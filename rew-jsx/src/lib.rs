#[derive(Debug)]
enum Attr {
  KeyValue(String, String),
  Spread(String),
}

#[derive(Debug)]
enum Node {
  Element {
    tag: String,
    attrs: Vec<Attr>,
    children: Vec<Node>,
  },
  Text(String),
  JSExpression(String),
}

struct Parser<'a> {
  input: &'a [u8],
  pos: usize,
}

impl<'a> Parser<'a> {
  fn new(input: &'a str) -> Self {
    Parser {
      input: input.as_bytes(),
      pos: 0,
    }
  }

  fn parse(&mut self) -> Vec<Node> {
    let mut nodes = Vec::new();
    while self.pos < self.input.len() {
      self.skip_whitespace();
      if self.starts_with("</") {
        break;
      } else if self.starts_with("<") {
        nodes.push(self.parse_element());
      } else {
        nodes.push(self.parse_text());
      }
    }
    nodes
  }

  fn parse_element(&mut self) -> Node {
    self.consume("<");
    let tag = self.consume_identifier();
    let attrs = self.parse_attributes();

    if self.starts_with("/>") {
      self.consume("/>");
      Node::Element {
        tag,
        attrs,
        children: Vec::new(),
      }
    } else {
      self.consume(">");
      let children = self.parse();
      self.consume("</");
      let end_tag = self.consume_identifier();
      assert_eq!(tag, end_tag, "Mismatched closing tag");
      self.consume(">");
      Node::Element {
        tag,
        attrs,
        children,
      }
    }
  }

  fn parse_braced_content(&mut self) -> Node {
    self.consume("{");
    let mut output = String::new();

    while self.pos < self.input.len() {
      if self.starts_with("}") {
        self.consume("}");
        break;
      } else if self.starts_with("<") {
        // parse JSX and compile immediately
        let jsx_node = self.parse_element();
        let compiled_jsx = compile_node(&jsx_node, None);
        output.push_str(&compiled_jsx);
      } else {
        output.push(self.advance());
      }
    }

    Node::JSExpression(output.trim().to_string())
  }

  fn parse_attributes(&mut self) -> Vec<Attr> {
    let mut attrs = Vec::new();
    loop {
      self.skip_whitespace();
      if self.peek() == '>' || self.starts_with("/>") {
        break;
      }

      if self.starts_with("{...") {
        self.consume("{...");
        let mut expr = String::new();
        while self.peek() != '}' {
          expr.push(self.advance());
        }
        self.consume("}");
        attrs.push(Attr::Spread(expr.trim().to_string()));
        continue;
      }

      let name = self.consume_identifier();
      self.skip_whitespace();

      let value = if self.starts_with("=") {
        self.consume("=");
        self.skip_whitespace();
        if self.starts_with("\"") {
          format!("\"{}\"", self.consume_quoted_string())
        } else if self.starts_with("{") {
          self.parse_braced_attribute()
        } else {
          panic!("Expected attribute value");
        }
      } else {
        // unvalued attribute -> default to true
        "true".to_string()
      };

      attrs.push(Attr::KeyValue(name, value));
    }
    attrs
  }

  fn parse_braced_attribute(&mut self) -> String {
    self.consume("{");
    let mut output = String::new();

    while self.pos < self.input.len() {
      if self.starts_with("}") {
        self.consume("}");
        break;
      } else if self.starts_with("<") {
        // Parse JSX and compile immediately
        let jsx_node = self.parse_element();
        output.push_str(&compile_node(&jsx_node, None));
      } else {
        output.push(self.advance());
      }
    }

    output.trim().to_string()
  }

  fn parse_text(&mut self) -> Node {
    let mut text = String::new();
    while self.pos < self.input.len() && !self.starts_with("<") && !self.starts_with("{") {
      text.push(self.advance());
    }
    if self.starts_with("{") {
      return self.parse_braced_content();
    }
    Node::Text(text.trim().to_string())
  }

  // Utilities

  fn starts_with(&self, s: &str) -> bool {
    self.input[self.pos..].starts_with(s.as_bytes())
  }

  fn peek(&self) -> char {
    self.input[self.pos] as char
  }

  fn advance(&mut self) -> char {
    let c = self.input[self.pos] as char;
    self.pos += 1;
    c
  }

  fn consume(&mut self, s: &str) {
    assert!(self.starts_with(s), "Expected '{}'", s);
    self.pos += s.len();
  }

  fn consume_identifier(&mut self) -> String {
    let mut ident = String::new();
    while self.pos < self.input.len() {
      let c = self.peek();
      if c.is_alphanumeric() || c == '-' || c == '_' {
        ident.push(self.advance());
      } else {
        break;
      }
    }
    ident
  }

  fn consume_quoted_string(&mut self) -> String {
    self.consume("\"");
    let mut value = String::new();
    while self.peek() != '"' {
      value.push(self.advance());
    }
    self.consume("\"");
    value
  }

  fn skip_whitespace(&mut self) {
    while self.pos < self.input.len() && self.peek().is_whitespace() {
      self.advance();
    }
  }
}

fn compile_node(node: &Node, pragma: Option<String>) -> String {
  match node {
    Node::Text(text) => {
      if text.trim().is_empty() {
        String::new()
      } else if text.starts_with('{') && text.ends_with('}') {
        // dynamic expression
        text[1..text.len() - 1].trim().to_string()
      } else {
        format!(r#""{}""#, text)
      }
    }
    Node::Element {
      tag,
      attrs,
      children,
    } => {
      let mut parts = Vec::new();
      for attr in attrs {
        match attr {
          Attr::KeyValue(k, v) => {
            if v.starts_with("jsx(") || v.contains("(") || v.contains("=>") || v.contains(".") {
              parts.push(format!(r#"{k}: {}"#, v)); // direct expression
            } else {
              parts.push(format!(r#"{k}: {}"#, v)); // string literal
            }
          }
          Attr::Spread(expr) => {
            parts.push(format!("...{}", expr));
          }
        }
      }
      let props = format!("{{{}}}", parts.join(", "));

      let compiled_children: Vec<String> = children
        .iter()
        .map(|x| compile_node(x, pragma.clone()))
        .filter(|c| !c.is_empty())
        .collect();
      let children_js = if compiled_children.is_empty() {
        "null".to_string()
      } else {
        compiled_children.join(", ")
      };

      let element = if tag == &tag.to_lowercase() {
        format!(r#""{}""#, tag)
      } else {
        tag.to_string()
      };

      let prefix = pragma.unwrap_or("JSX.prototype.new".to_string());

      format!(r#"{prefix}({element}, {props}, {children_js})"#)
    }
    Node::JSExpression(code) => code.clone(),
  }
}

#[derive(Debug, Clone)]
enum Token {
  Symbol(String),
  String(String),
  Identifier(String),
  Comment(String),
  Whitespace(String),
  #[allow(unused)]
  Other(String),
}

fn tokenize(source: &str) -> Vec<Token> {
  let mut tokens = vec![];
  let mut chars = source.chars().peekable();

  while let Some(&c) = chars.peek() {
    if c.is_whitespace() {
      let mut ws = String::new();
      while let Some(&c2) = chars.peek() {
        if c2.is_whitespace() {
          ws.push(c2);
          chars.next();
        } else {
          break;
        }
      }
      tokens.push(Token::Whitespace(ws));
    } else if c == '"' || c == '\'' {
      let quote = c;
      let mut s = String::new();
      s.push(c);
      chars.next();
      for ch in chars.by_ref() {
        s.push(ch);
        if ch == quote {
          break;
        }
      }
      tokens.push(Token::String(s));
    } else if c == '/' && chars.clone().nth(1) == Some('/') {
      let mut comment = String::new();
      for ch in chars.by_ref() {
        comment.push(ch);
        if ch == '\n' {
          break;
        }
      }
      tokens.push(Token::Comment(comment));
    } else if c.is_alphanumeric() || c == '_' {
      let mut ident = String::new();
      while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
          ident.push(ch);
          chars.next();
        } else {
          break;
        }
      }
      tokens.push(Token::Identifier(ident));
    } else {
      let mut sym = String::new();
      sym.push(c);
      chars.next();

      // Look ahead for compound symbols
      if sym == "<" && chars.peek() == Some(&'/') {
        sym.push('/');
        chars.next();
      }

      tokens.push(Token::Symbol(sym));
    }
  }

  tokens
}

fn compile_jsx_fragments(tokens: &[Token], pragma: Option<String>) -> String {
  let mut output = String::new();
  let mut i = 0;

  while i < tokens.len() {
    // complex bullshit over here
    let is_open = if let Token::Symbol(ref sym1) = tokens[i] {
      if sym1 == "<" {
        if let Some(Token::Symbol(sym2)) = tokens.get(i + 1) {
          sym2 == ">"
        } else {
          false
        }
      } else {
        false
      }
    } else {
      false
    };

    if is_open {
      // fuck < and >
      i += 2;

      let mut jsx = String::new();
      while i + 1 < tokens.len() {
        // complex stuff
        let is_close = if let Token::Symbol(sym1) = &tokens[i] {
          if sym1 == "</" {
            if let Some(Token::Symbol(sym2)) = tokens.get(i + 1) {
              sym2 == ">"
            } else {
              false
            }
          } else {
            false
          }
        } else {
          false
        };

        if is_close {
          // fuck < and />
          i += 2;
          break; // fuck off
        }

        match &tokens[i] {
          Token::Whitespace(s)
          | Token::Identifier(s)
          | Token::Symbol(s)
          | Token::String(s)
          | Token::Comment(s)
          | Token::Other(s) => jsx.push_str(s),
        }
        i += 1;
      }

      let mut parser = Parser::new(&jsx);
      for node in parser.parse() {
        output.push_str(&compile_node(&node, pragma.clone()));
      }

      continue;
    }

    match &tokens[i] {
      Token::Whitespace(s)
      | Token::Identifier(s)
      | Token::Symbol(s)
      | Token::String(s)
      | Token::Comment(s)
      | Token::Other(s) => output.push_str(s),
    }
    i += 1;
  }

  output
}

pub fn compile_jsx(input: String, pragma: Option<String>) -> String {
  compile_jsx_fragments(&tokenize(&input), pragma)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_jsx() {
    assert_eq!(
      compile_jsx(
        "<><div>{something.map((i) => <p>{i}</p>)}<Element name={<i>{u}</i>} /></div></>".into(),
        None
      ),
      "JSX.prototype.new(\"div\", {}, something.map((i) => JSX.prototype.new(\"p\", {}, i)), JSX.prototype.new(Element, {name: JSX.prototype.new(\"i\", {}, u)}, null))"
    )
  }
}
