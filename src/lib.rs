pub trait Element {
    fn render(&self) -> String;
}

pub trait Container<'a> {
    fn with<E: Element + 'a>(self, e: E) -> Self;
}

pub struct Document<'a> {
    preambule: Preambule,
    body: Boxed<'a>,
}

impl Document<'_> {
    pub fn new() -> Self {
        let mut body = Boxed::new();
        body.prep = body.prep.with(Macros::new("begin").param("document"));
        body.after = body.after.with(Macros::new("end").param("document"));

        Self {
            preambule: Preambule::new(),
            body,
        }
    }

    pub fn preambule(&mut self) -> &mut Preambule {
        &mut self.preambule
    }
}

impl<'a> Container<'a> for Document<'a> {
    fn with<E: Element + 'a>(mut self, e: E) -> Self {
        self.body.middle = self.body.middle.with(e);
        self
    }
}

impl Element for Document<'_> {
    fn render(&self) -> String {
        self.preambule.render() + "\n\n" + &self.body.render() + "\n"
    }
}

pub struct Macros {
    m: String,
    params: Vec<Parameter>,
}

impl Macros {
    pub fn new<S: AsRef<str>>(m: S) -> Self {
        Self {
            m: m.as_ref().to_owned(),
            params: Vec::new(),
        }
    }

    pub fn param<P: Into<Parameter>>(mut self, parameter: P) -> Self {
        self.params.push(parameter.into());
        self
    }
}

impl Element for Macros {
    fn render(&self) -> String {
        let params = self.params.iter().map(|p| p.render()).collect::<String>();
        if !params.is_empty() {
            format!("\\{}{{{}}}", self.m.clone(), params)
        } else {
            format!("\\{}", self.m.clone())
        }
    }
}

pub struct Text<S: AsRef<str>>(S);

impl<S: AsRef<str>> Element for Text<S> {
    fn render(&self) -> String {
        self.0.as_ref().to_owned()
    }
}

pub struct Preambule {
    r#type: DocumentType,
    author: Option<Parameter>,
    tittle: Option<Parameter>,
}

pub enum DocumentType {
    Article,
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentType::Article => f.write_str("article"),
        }
    }
}

impl Preambule {
    pub fn new() -> Self {
        Self {
            r#type: DocumentType::Article,
            author: None,
            tittle: None,
        }
    }

    pub fn r#type(&mut self, t: DocumentType) -> &mut Self {
        self.r#type = t;
        self
    }

    pub fn tittle<P>(&mut self, parameter: P) -> &mut Self
    where
        P: Into<Parameter>,
    {
        self.tittle = Some(Macros::new("title").param(parameter.into()).into());
        self
    }

    pub fn author<S>(&mut self, author: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.author = Some(Macros::new("author").param(author).into());
        self
    }
}

impl Element for Preambule {
    fn render(&self) -> String {
        let mut buf = Vec::new();

        buf.push(format!("\\documentclass{{{}}}", self.r#type));

        self.tittle.as_ref().map(|tittle| buf.push(tittle.render()));
        self.author.as_ref().map(|author| buf.push(author.render()));

        buf.join("\n")
    }
}

#[allow(non_snake_case)]
pub fn LaTeX() -> Macros {
    Macros::new("LaTeX")
}

pub struct Boxed<'a> {
    prep: Area<'a>,
    middle: Area<'a>,
    after: Area<'a>,
}

impl Boxed<'_> {
    fn new() -> Self {
        Self {
            prep: Area::new(),
            middle: Area::new(),
            after: Area::new(),
        }
    }
}

impl<'a> Container<'a> for Boxed<'a> {
    fn with<E: Element + 'a>(self, e: E) -> Self {
        self
    }
}

impl Element for Boxed<'_> {
    fn render(&self) -> String {
        self.prep.render() + "\n" + &self.middle.render() + "\n" + &self.after.render()
    }
}

pub struct Area<'a> {
    objs: Vec<Box<dyn Element + 'a>>,
}

impl Area<'_> {
    pub fn new() -> Self {
        Self { objs: Vec::new() }
    }
}

impl<'a> Container<'a> for Area<'a> {
    fn with<E: Element + 'a>(mut self, e: E) -> Self {
        self.objs.push(Box::new(e));
        self
    }
}

impl Element for Area<'_> {
    fn render(&self) -> String {
        self.objs.iter().map(|obj| obj.render()).collect()
    }
}

pub enum Parameter {
    Literal(String),
    Macros(Macros),
}

impl<S: AsRef<str>> From<S> for Parameter {
    fn from(s: S) -> Parameter {
        Parameter::Literal(s.as_ref().to_owned())
    }
}

impl Into<Parameter> for Macros {
    fn into(self) -> Parameter {
        Parameter::Macros(self)
    }
}

impl Element for Parameter {
    fn render(&self) -> String {
        match self {
            Self::Literal(l) => l.clone(),
            Self::Macros(m) => m.render(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected = r"\documentclass{article}
\title{\LaTeX}
\author{Maxim Zhiburt}

\begin{document}
something
\end{document}
";

        let mut doc = Document::new();
        doc.preambule.tittle(LaTeX()).author("Maxim Zhiburt");

        let doc = doc.with(Text("something"));

        let rendered = doc.render();
        assert_eq!(expected, rendered)
    }
}
