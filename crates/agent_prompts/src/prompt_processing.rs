use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex to match placeholders. The pattern matches antyhing between "{{" and "}}". No new line is allowed in the placeholder name.
    ///
    /// TODO: when `LazyCell` is stabilized, use that instead
    pub(crate) static ref PLACEHOLDER_MATCH_RE: Regex = Regex::new(r"\{\{.*?\}\}").unwrap();
}

pub struct PromptTemplate {
    src: String,
    matches: Vec<(usize, usize)>,
}

impl PromptTemplate {
    pub fn new(template: &str) -> Self {
        // let regex = Regex::new(r"\{\{([^}]*)\}\}").unwrap();

        PromptTemplate {
            src: template.to_owned(),
            matches: PLACEHOLDER_MATCH_RE
                .find_iter(template)
                .map(|m| (m.start(), m.end()))
                .collect(),
        }
    }

    /// ```
    /// # Examples
    ///
    /// let template = Template::new("Hi, my name is {{name}} and I'm a {{lang}} developer.");
    ///
    /// let mut args = HashMap::new();
    /// args.insert("name", "Michael");
    /// args.insert("lang", "Rust");
    /// let s = template.render(&args);
    ///
    /// assert_eq!(s, "Hi, my name is Michael and I'm a Rust developer.");
    ///
    /// let mut args1 = HashMap::new();
    /// args1.insert("name", "Vader");
    /// args1.insert("lang", "Dart");
    /// let s2 = template.render(&args1);
    ///
    /// assert_eq!(s2, "Hi, my name is Vader and I'm a Dart developer.");
    /// ```
    pub fn render(&self, vals: &HashMap<&str, &str>) -> String {
        self.render_named(vals)
    }

    ///
    /// See render() for examples.
    ///
    pub fn render_named(&self, vals: &HashMap<&str, &str>) -> String {
        let mut parts: Vec<&str> = vec![];
        let template_str = &self.src;

        // get index of first arg match or return a copy of the template if no args matched
        let first = match self.matches.first() {
            Some((start, _)) => *start,
            _ => return template_str.clone(),
        };

        // copy from template start to first arg
        if first > 0 {
            parts.push(&template_str[0..first])
        }

        // keeps the index of the previous argument end
        let mut prev_end: Option<usize> = None;

        for (start, end) in self.matches.iter() {
            // copy from previous argument end till current argument start
            if let Some(last_end) = prev_end {
                parts.push(&template_str[last_end..*start])
            }

            // argument name with braces
            let arg = &template_str[*start..*end];
            // just the argument name
            let arg_name = &arg[2..arg.len() - 2];

            // if value passed for argument then append it, otherwise append original argument
            // name with braces
            match vals.get(arg_name) {
                Some(s) => parts.push(s),
                _ => parts.push(arg),
            }

            prev_end = Some(*end);
        }

        let template_len = template_str.len();
        // if last arg end index isn't the end of the string then copy
        // from last arg end till end of template string
        if let Some(last_pos) = prev_end {
            if last_pos < template_len {
                parts.push(&template_str[last_pos..template_len])
            }
        }

        parts.join("")
    }

    /// ```
    /// let template = Template::new("Hi, my name is {{}} and I'm a {{}} developer.");
    ///
    /// let args = vec!["Michael", "Rust"];
    /// let s = template.render_positional(&args);
    /// assert_eq!(s, "Hi, my name is Michael and I'm a Rust developer.");
    ///
    /// let args1 = vec!["Vader", "Dart"];
    /// let s2 = template.render_positional(&args1);
    /// assert_eq!(s2, "Hi, my name is Vader and I'm a Dart developer.");
    /// ```
    pub fn render_positional(&self, vals: &[&str]) -> String {
        let mut parts: Vec<&str> = vec![];
        let template_str = &self.src;

        // get index of first arg match or return a copy of the template if no args matched
        let first = match self.matches.first() {
            Some((start, _)) => *start,
            _ => return template_str.clone(),
        };

        // copy from template start to first arg
        if first > 0 {
            parts.push(&template_str[0..first])
        }

        // keeps the index of the previous argument end
        let mut prev_end: Option<usize> = None;

        for ((start, end), val) in self.matches.iter().zip(vals.iter()) {
            // copy from previous argument end till current argument start
            if let Some(last_end) = prev_end {
                parts.push(&template_str[last_end..*start])
            }

            parts.push(val);

            prev_end = Some(*end);
        }

        let template_len = template_str.len();
        // if last arg end index isn't the end of the string then copy
        // from last arg end till end of template string
        if let Some(last_pos) = prev_end {
            if last_pos < template_len {
                parts.push(&template_str[last_pos..template_len])
            }
        }

        parts.join("")
    }
}