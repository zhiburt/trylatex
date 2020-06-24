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
        body.prep = body.prep.with(Literal("\\begin{document}"));
        body.after = body.after.with(Literal("\\end{document}"));

        Self {
            preambule: Preambule::default(),
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

pub struct Literal<S: AsRef<str>>(S);

impl<S: AsRef<str>> Element for Literal<S> {
    fn render(&self) -> String {
        self.0.as_ref().to_owned()
    }
}

#[derive(Default)]
pub struct Preambule {
    r#type: Option<Literal<String>>,
    author: Option<Literal<String>>,
    tittle: Option<Literal<String>>,
}

impl Preambule {
    pub fn r#type<L, S>(&mut self, s: L) -> &mut Self
    where
        L: Into<Literal<S>>,
        S: AsRef<str>,
    {
        let literal = s.into();
        self.r#type = Some(Literal(literal.0.as_ref().to_owned()));
        self
    }

    pub fn tittle<L, S>(&mut self, s: L) -> &mut Self
    where
        L: Into<Literal<S>>,
        S: AsRef<str>,
    {
        let literal = s.into();
        self.tittle = Some(Literal(literal.0.as_ref().to_owned()));
        self
    }

    pub fn author<L, S>(&mut self, s: L) -> &mut Self
    where
        L: Into<Literal<S>>,
        S: AsRef<str>,
    {
        let literal = s.into();
        self.author = Some(Literal(literal.0.as_ref().to_owned()));
        self
    }
}

impl Element for Preambule {
    fn render(&self) -> String {
        let mut buf = Vec::new();

        let tp = self
            .r#type
            .as_ref()
            .map_or("article".to_string(), |tp| tp.0.to_string());
        buf.push(format!("\\documentclass{{{}}}", tp));

        self.tittle
            .as_ref()
            .map(|tittle| buf.push(format!("\\title{{{}}}", tittle.0)));

        self.author
            .as_ref()
            .map(|author| buf.push(format!("\\author{{{}}}", author.0)));

        buf.join("\n")
    }
}

#[allow(non_snake_case)]
pub fn Tittle<S: AsRef<str>>(tittle: S) -> Literal<S> {
    Literal(tittle)
}

#[allow(non_snake_case)]
pub fn Author<S: AsRef<str>>(tittle: S) -> Literal<S> {
    Literal(tittle)
}

#[allow(non_snake_case)]
pub fn Text<S: AsRef<str>>(tittle: S) -> Literal<S> {
    Literal(tittle)
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
        doc.preambule
            .r#type(Literal("article"))
            .tittle(Literal("\\LaTeX"))
            .author(Literal("Maxim Zhiburt"));

        let doc = doc.with(Text("something"));

        let rendered = doc.render();
        assert_eq!(expected, rendered)
    }
}
