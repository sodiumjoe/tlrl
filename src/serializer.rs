use html5ever::{
    serialize::{AttrRef, Serialize, SerializeOpts, Serializer, TraversalScope},
    LocalName, QualName,
};
use std::default::Default;
use std::io::{self, Write};

pub fn serialize<Wr, T>(writer: Wr, node: &T, opts: SerializeOpts) -> io::Result<()>
where
    Wr: Write,
    T: Serialize,
{
    let mut ser = HtmlSerializer::new(writer, opts.clone());
    node.serialize(&mut ser, opts.traversal_scope)
}

#[derive(Default)]
struct ElemInfo {
    html_name: Option<LocalName>,
    ignore_children: bool,
    processed_first_child: bool,
}

struct HtmlSerializer<Wr: Write> {
    writer: Wr,
    opts: SerializeOpts,
    stack: Vec<ElemInfo>,
}

fn tagname(name: &QualName) -> LocalName {
    match name.ns {
        ns!(html) | ns!(mathml) | ns!(svg) => (),
        ref ns => {
            // FIXME(#122)
            warn!("node with weird namespace {:?}", ns);
        }
    }

    name.local.clone()
}

impl<Wr: Write> HtmlSerializer<Wr> {
    fn new(writer: Wr, opts: SerializeOpts) -> Self {
        let html_name = match opts.traversal_scope {
            TraversalScope::IncludeNode | TraversalScope::ChildrenOnly(None) => None,
            TraversalScope::ChildrenOnly(Some(ref n)) => Some(tagname(n)),
        };
        HtmlSerializer {
            writer: writer,
            opts: opts,
            stack: vec![ElemInfo {
                html_name: html_name,
                ignore_children: false,
                processed_first_child: false,
            }],
        }
    }

    fn parent(&mut self) -> &mut ElemInfo {
        if self.stack.len() == 0 {
            if self.opts.create_missing_parent {
                warn!("ElemInfo stack empty, creating new parent");
                self.stack.push(Default::default());
            } else {
                panic!("no parent ElemInfo")
            }
        }
        self.stack.last_mut().unwrap()
    }

    fn write_escaped(&mut self, text: &str, attr_mode: bool) -> io::Result<()> {
        for c in text.chars() {
            try!(match c {
                '&' => self.writer.write_all(b"&amp;"),
                '\u{00A0}' => self.writer.write_all(b"&nbsp;"),
                '\u{2002}' => self.writer.write_all(b"&ensp;"),
                '\u{2003}' => self.writer.write_all(b"&emsp;"),
                '\u{2009}' => self.writer.write_all(b"&thinsp;"),
                '\u{200C}' => self.writer.write_all(b"&zwnj;"),
                '\u{200D}' => self.writer.write_all(b"&zwj;"),
                '\u{200E}' => self.writer.write_all(b"&lrm;"),
                '\u{200F}' => self.writer.write_all(b"&rlm;"),
                '–' => self.writer.write_all(b"&ndash;"),
                '—' => self.writer.write_all(b"&mdash;"),
                '‘' => self.writer.write_all(b"&lsquo;"),
                '’' => self.writer.write_all(b"&rsquo;"),
                '‚' => self.writer.write_all(b"&sbquo;"),
                '“' => self.writer.write_all(b"&ldquo;"),
                '”' => self.writer.write_all(b"&rdquo;"),
                '„' => self.writer.write_all(b"&bdquo;"),
                '†' => self.writer.write_all(b"&dagger;"),
                '‡' => self.writer.write_all(b"&Dagger;"),
                '‰' => self.writer.write_all(b"&permil;"),
                '‹' => self.writer.write_all(b"&lsaquo;"),
                '›' => self.writer.write_all(b"&rsaquo;"),
                '•' => self.writer.write_all(b"&bull;"),
                '…' => self.writer.write_all(b"&hellip;"),
                '′' => self.writer.write_all(b"&prime;"),
                '″' => self.writer.write_all(b"&Prime;"),
                '‾' => self.writer.write_all(b"&oline;"),
                '⁄' => self.writer.write_all(b"&frasl;"),
                '"' if attr_mode => self.writer.write_all(b"&quot;"),
                '<' if !attr_mode => self.writer.write_all(b"&lt;"),
                '>' if !attr_mode => self.writer.write_all(b"&gt;"),
                c => self.writer.write_fmt(format_args!("{}", c)),
            });
        }
        Ok(())
    }
}

impl<Wr: Write> Serializer for HtmlSerializer<Wr> {
    fn start_elem<'a, AttrIter>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
    where
        AttrIter: Iterator<Item = AttrRef<'a>>,
    {
        let html_name = match name.ns {
            ns!(html) => Some(name.local.clone()),
            _ => None,
        };

        if self.parent().ignore_children {
            self.stack.push(ElemInfo {
                html_name: html_name,
                ignore_children: true,
                processed_first_child: false,
            });
            return Ok(());
        }

        try!(self.writer.write_all(b"<"));
        try!(self.writer.write_all(tagname(&name).as_bytes()));
        for (name, value) in attrs {
            try!(self.writer.write_all(b" "));

            match name.ns {
                ns!() => (),
                ns!(xml) => try!(self.writer.write_all(b"xml:")),
                ns!(xmlns) => {
                    if name.local != local_name!("xmlns") {
                        try!(self.writer.write_all(b"xmlns:"));
                    }
                }
                ns!(xlink) => try!(self.writer.write_all(b"xlink:")),
                ref ns => {
                    // FIXME(#122)
                    warn!("attr with weird namespace {:?}", ns);
                    try!(self.writer.write_all(b"unknown_namespace:"));
                }
            }

            try!(self.writer.write_all(name.local.as_bytes()));
            try!(self.writer.write_all(b"=\""));
            try!(self.write_escaped(value, true));
            try!(self.writer.write_all(b"\""));
        }
        try!(self.writer.write_all(b">"));

        let ignore_children = name.ns == ns!(html)
            && match name.local {
                local_name!("area")
                | local_name!("base")
                | local_name!("basefont")
                | local_name!("bgsound")
                | local_name!("br")
                | local_name!("col")
                | local_name!("embed")
                | local_name!("frame")
                | local_name!("hr")
                | local_name!("img")
                | local_name!("input")
                | local_name!("keygen")
                | local_name!("link")
                | local_name!("meta")
                | local_name!("param")
                | local_name!("source")
                | local_name!("track")
                | local_name!("wbr") => true,
                _ => false,
            };

        self.parent().processed_first_child = true;

        self.stack.push(ElemInfo {
            html_name: html_name,
            ignore_children: ignore_children,
            processed_first_child: false,
        });

        Ok(())
    }

    fn end_elem(&mut self, name: QualName) -> io::Result<()> {
        let info = match self.stack.pop() {
            Some(info) => info,
            None if self.opts.create_missing_parent => {
                warn!("missing ElemInfo, creating default.");
                Default::default()
            }
            _ => panic!("no ElemInfo"),
        };
        if info.ignore_children {
            return Ok(());
        }

        try!(self.writer.write_all(b"</"));
        try!(self.writer.write_all(tagname(&name).as_bytes()));
        self.writer.write_all(b">")
    }

    fn write_text(&mut self, text: &str) -> io::Result<()> {
        let escape = match self.parent().html_name {
            Some(local_name!("style"))
            | Some(local_name!("script"))
            | Some(local_name!("xmp"))
            | Some(local_name!("iframe"))
            | Some(local_name!("noembed"))
            | Some(local_name!("noframes"))
            | Some(local_name!("plaintext")) => false,

            Some(local_name!("noscript")) => !self.opts.scripting_enabled,

            _ => true,
        };

        if escape {
            self.write_escaped(text, false)
        } else {
            self.writer.write_all(text.as_bytes())
        }
    }

    fn write_comment(&mut self, text: &str) -> io::Result<()> {
        try!(self.writer.write_all(b"<!--"));
        try!(self.writer.write_all(text.as_bytes()));
        self.writer.write_all(b"-->")
    }

    fn write_doctype(&mut self, name: &str) -> io::Result<()> {
        try!(self.writer.write_all(b"<!DOCTYPE "));
        try!(self.writer.write_all(name.as_bytes()));
        self.writer.write_all(b">")
    }

    fn write_processing_instruction(&mut self, target: &str, data: &str) -> io::Result<()> {
        try!(self.writer.write_all(b"<?"));
        try!(self.writer.write_all(target.as_bytes()));
        try!(self.writer.write_all(b" "));
        try!(self.writer.write_all(data.as_bytes()));
        self.writer.write_all(b">")
    }
}
