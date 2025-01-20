use regex::Regex;

#[derive(Debug, Clone)]
pub enum CalloutKind {
    Note,
    Warning,
    Info,
    Error,
}

#[derive(Debug)]
pub enum ToDoKind {
    Task,
    Done,
}

pub enum BlockToken {
    // Single Line tokens
    Header {
        level: usize,
        text: String,
    }, // # Header text
    AudioStamp(String), // @04:45
    Image {
        alt: String,
        url: String,
        width: String,
        reference: String,
        caption: String,
    }, // ![alt](url "width") {ref: id} {cap: caption}
    FootNote {
        id: String,
        description: String,
    }, // [^1]: Footnote text
    ToDo {
        kind: ToDoKind,
        description: String,
    }, // [ ] Task, [x] Done
    HorizontalLine,     // ---

    // Accumulating tokens
    Paragraph(String), // A block of text
    List(String),      // :::list
    Table {
        id: String,
        content: String,
    }, // :::table
    Callout {
        kind: CalloutKind,
        info: String,
    }, // !!! Note, !!! Warning
    Collapse(String),  // :::collapse
    BlockMath {
        id: String,
        content: String,
    }, // $$E=mc^2$$
    Canvas {
        id: String,
        content: String,
    }, // :::canvas
    ImageCarousel {
        id: String,
        content: String,
    }, // :::carousel
    Question {
        id: String,
        content: String,
    }, // :::question
    Video {
        id: String,
        content: String,
    }, // :::video
    Quote {
        id: String,
        content: String,
    }, // :::quote
    FAQ {
        id: String,
        content: String,
    }, // :::faq
    CodeBlock {
        lang: String,
        code: String,
    }, // ```rust\nfn main() {}\n```
}

#[derive(PartialEq)]
pub enum AccumulatingType {
    Paragraph,
    List,
    Table,
    Callout,
    Collapse,
    BlockMath,
    Canvas,
    ImageCarousel,
    Question,
    Video,
    Quote,
    FAQ,
    CodeBlock,
}

fn flush_accumulator(
    accumulator: &mut String,
    accumulator_type: &AccumulatingType,
    callout_kind: &CalloutKind,
    tokens: &mut Vec<BlockToken>,
) {
    if accumulator.is_empty() {
        return;
    }
    match accumulator_type {
        AccumulatingType::Paragraph => {
            tokens.push(BlockToken::Paragraph(accumulator.clone()));
        }
        AccumulatingType::List => {
            tokens.push(BlockToken::List(accumulator.clone()));
        }
        AccumulatingType::Table => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::Table { id, content });
        }
        AccumulatingType::Callout => {
            tokens.push(BlockToken::Callout {
                kind: callout_kind.clone(),
                info: accumulator.clone(),
            });
        }
        AccumulatingType::Collapse => {
            tokens.push(BlockToken::Collapse(accumulator.clone()));
        }
        AccumulatingType::BlockMath => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::BlockMath { id, content });
        }
        AccumulatingType::Canvas => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::Canvas { id, content });
        }
        AccumulatingType::ImageCarousel => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::ImageCarousel { id, content });
        }
        AccumulatingType::Question => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::Question { id, content });
        }
        AccumulatingType::Video => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::Video { id, content });
        }
        AccumulatingType::Quote => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::Quote { id, content });
        }
        AccumulatingType::FAQ => {
            let mut parts = accumulator.splitn(2, '\n');
            let id = parts.next().unwrap_or("").to_string();
            let content = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::FAQ { id, content });
        },
        AccumulatingType::CodeBlock => {
            let mut parts = accumulator.splitn(2, '\n');
            let lang = parts.next().unwrap_or("").to_string();
            let code = parts.next().unwrap_or("").to_string();
            tokens.push(BlockToken::CodeBlock { lang, code });
        }
    }
    accumulator.clear();
}

pub fn tokenizer(input: &str) -> Vec<BlockToken> {
    let mut tokens: Vec<BlockToken> = Vec::new();

    let mut accumulator: String = String::new();
    let mut accumulator_type: AccumulatingType = AccumulatingType::Paragraph;
    let mut callout_kind: CalloutKind = CalloutKind::Info;

    for line in input.lines() {
        let trimmed = line.trim();

        // If the line starts with ``` and the accumulator is empty, it's a code block
        if trimmed.starts_with("```") {
            // if the type is already code block, that means the code block has ended
            if accumulator_type == AccumulatingType::CodeBlock {
                flush_accumulator(
                    &mut accumulator,
                    &accumulator_type,
                    &callout_kind,
                    &mut tokens,
                );
                accumulator_type = AccumulatingType::Paragraph;
                continue;
            }

            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::CodeBlock;
            let lang = trimmed.chars().skip(3).collect::<String>().trim().to_string().to_lowercase();
            accumulator.push_str(&lang);
            accumulator.push('\n');
        }

        // code block can have anything.... until it ends
        else if accumulator_type == AccumulatingType::CodeBlock {
            accumulator.push_str(line);
            accumulator.push('\n');
            continue;
        }

        // If the line is empty, flush the accumulator
        else if trimmed.is_empty() {
            continue;
        }

        // if starts with #, it's a header
        else if trimmed.starts_with("#") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
            let level = trimmed.chars().take_while(|c| *c == '#').count();
            let text = trimmed.chars().skip(level).collect();
            tokens.push(BlockToken::Header { level, text });
        }
        // if starts with @, it's an audio stamp
        else if trimmed.starts_with("@") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
            let text = trimmed.chars().skip(1).collect();
            tokens.push(BlockToken::AudioStamp(text));
        }
        // If the line starts with ![, it's an image
        else if trimmed.starts_with("![") {
            // If the accumulator is carousel, store the line in the accumulator
            if accumulator_type == AccumulatingType::ImageCarousel {
                accumulator.push_str(line);
                accumulator.push('\n');
                continue;
            }
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
        
            // Extract the part between ![ and ] for the alt text
            let alt = trimmed
                .split(']')
                .next()
                .unwrap_or("")
                .strip_prefix("![")
                .unwrap_or("")
                .trim()
                .to_string();
        
            // Extract the URL and optional width inside parentheses ()
            let mut url = String::new();
            let mut width = String::new();
            if let Some(parentheses) = trimmed.split(')').next() {
                let inner = parentheses.split('(').nth(1).unwrap_or("").trim();
                let mut parts = inner.split_whitespace();
                url = parts.next().unwrap_or("").to_string();
                width = parts.next().unwrap_or("").trim_matches('"').to_string();
            }
        
            // Extract the optional reference and caption
            let mut reference = String::new();
            let mut caption = String::new();
            for part in trimmed.split('{').skip(1) {
                let cleaned = part.trim_end_matches('}').trim();
                if cleaned.starts_with("ref:") {
                    reference = cleaned.strip_prefix("ref:").unwrap_or("").trim().to_string();
                } else if cleaned.starts_with("cap:") {
                    caption = cleaned.strip_prefix("cap:").unwrap_or("").trim().to_string();
                }
            }
            // remove trail } from reference if it exists
            reference = reference.trim_end_matches('}').to_string();
            caption = caption.trim_end_matches('}').to_string();
        
            // Push the parsed image token to tokens
            tokens.push(BlockToken::Image {
                alt,
                url,
                width,
                reference,
                caption,
            });
        }
        
        // if starts with [^, it's a footnote
        else if trimmed.starts_with("[^") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
        
            let mut parts = trimmed.splitn(2, ':');
            let id = parts
                .next()
                .unwrap()
                .chars()
                .skip(2)
                .collect::<String>()
                .trim_end_matches(']')
                .to_string();
        
            let description = parts.next().unwrap().chars().skip(1).collect();
        
            tokens.push(BlockToken::FootNote { id, description });
        }
        // if starts with [ ], it's a task
        else if trimmed.starts_with("[ ]") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            let description = trimmed.chars().skip(3).collect();
            tokens.push(BlockToken::ToDo {
                kind: ToDoKind::Task,
                description,
            });
        }
        // if starts with [x], it's a done task
        else if trimmed.starts_with("[x]") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
            let description = trimmed.chars().skip(3).collect();
            tokens.push(BlockToken::ToDo {
                kind: ToDoKind::Done,
                description,
            });
        }
        // if starts with ---, it's a horizontal line
        else if trimmed.starts_with("---") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
            tokens.push(BlockToken::HorizontalLine);
        }

        // if starts with :::list, it's a list
        else if trimmed.starts_with(":::list") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::List;
        }
        // if starts with :::table, it's a table
        else if trimmed.starts_with(":::table") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Table;
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
        }
        // if starts with !!!, it's a callout
        else if trimmed.starts_with("!!!") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            let kind = match trimmed[3..].trim_start().to_lowercase().as_str() {
                "note" => CalloutKind::Note,
                "warning" => CalloutKind::Warning,
                "info" => CalloutKind::Info,
                "error" => CalloutKind::Error,
                _ => CalloutKind::Info,
            };
            callout_kind = kind;
            accumulator_type = AccumulatingType::Callout;
        }
        // if starts with :::collapse, it's a collapse
        else if trimmed.starts_with(":::collapse") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Collapse;
        }
        // if starts with $$, it's a block math
        else if trimmed.starts_with("$$") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            // If the type was already BlockMath, that means the block math has ended
            if accumulator_type == AccumulatingType::BlockMath {
                accumulator_type = AccumulatingType::Paragraph;
                continue;
            }
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
            accumulator_type = AccumulatingType::BlockMath;
        }
        // if starts with :::canvas, it's a canvas
        else if trimmed.starts_with(":::canvas") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Canvas;
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
        }
        // if starts with :::carousel, it's an image carousel
        else if trimmed.starts_with(":::carousel") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::ImageCarousel;
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
        }
        // if starts with :::question, it's a question
        else if trimmed.starts_with(":::question") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Question;
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
        }
        // if starts with :::video, it's a video
        else if trimmed.starts_with(":::video") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Video;
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
        }
        // if starts with :::quote, it's a quote
        else if trimmed.starts_with(":::quote") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Quote;
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
        }
        // if starts with :::faq, it's a FAQ
        else if trimmed.starts_with(":::faq") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::FAQ;
            let id = if let Some(after_ref) = trimmed.split("ref:").nth(1) {
                if let Some(id) = after_ref.split('}').next() {
                    id.to_string() // Extracted ID as a string
                } else {
                    "0".to_string() // Default to "0" if '}' is not found
                }
            } else {
                "0".to_string() // Default to "0" if "ref:" is not found
            };
            accumulator.push_str(&id);
            accumulator.push('\n');
        } else if trimmed.starts_with(":::") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
        } else if trimmed.starts_with("!!!") {
            flush_accumulator(
                &mut accumulator,
                &accumulator_type,
                &callout_kind,
                &mut tokens,
            );
            accumulator_type = AccumulatingType::Paragraph;
        } else {
            // If none of the above, add the line to the accumulator
            accumulator.push_str(line);
            accumulator.push('\n');
        }
    }

    // Flush the accumulator one last time
    flush_accumulator(
        &mut accumulator,
        &accumulator_type,
        &callout_kind,
        &mut tokens,
    );
    tokens
}

// Utility function to convert tokens to a single string
pub fn tokens_to_string(tokens: &Vec<BlockToken>) -> String {
    let mut result = String::new();

    for token in tokens {
        let token_str = match token {
            BlockToken::Header { level, text } => {
                format!("Header:\nlevel={}, text={}\n", level, text)
            }
            BlockToken::AudioStamp(text) => {
                format!("AudioStamp:\n{}\n", text)
            }
            BlockToken::Image {
                alt,
                url,
                width,
                reference,
                caption,
            } => {
                format!(
                    "Image:\nalt={}, url={}, width={}, reference={}, caption={}\n",
                    alt, url, width, reference, caption
                )
            }
            BlockToken::FootNote { id, description } => {
                format!("FootNote:\nid={}\n, description={}\n", id, description)
            }
            BlockToken::ToDo { kind, description } => {
                format!("ToDo:\nkind={:?}, description={}\n", kind, description)
            }
            BlockToken::HorizontalLine => "HorizontalLine\n".to_string(),
            BlockToken::Paragraph(text) => {
                format!("Paragraph:\n{}\n", text)
            }
            BlockToken::List(text) => {
                format!("List:\n{}\n", text)
            }
            BlockToken::Table { id, content } => {
                format!("Table:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::Callout { kind, info } => {
                format!("Callout:\nkind={:?},\ninfo={}\n", kind, info)
            }
            BlockToken::Collapse(text) => {
                format!("Collapse:\n{}\n", text)
            }
            BlockToken::BlockMath { id, content } => {
                format!("BlockMath:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::Canvas { id, content } => {
                format!("Canvas:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::ImageCarousel { id, content } => {
                format!("ImageCarousel:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::Question { id, content } => {
                format!("Question:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::Video { id, content } => {
                format!("Video:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::Quote { id, content } => {
                format!("Quote:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::FAQ { id, content } => {
                format!("FAQ:\nid={},\ncontent={}\n", id, content)
            }
            BlockToken::CodeBlock { lang, code } => {
                format!("CodeBlock:\nlang={},\ncode={}\n", lang, code)
            }
        };
        result.push_str(&token_str);
        result.push('\n');
    }

    result
}


// Text format tokens
// Works on paragraphs, lists, tables, callouts, collapses, questions, quotes and FAQs

// pub enum FormatToken {
//     Highlight{color: String, text: String}, // ==highlighted text=={color:red}
//     ColoredText{color: String, text: String}, // ===colored text==={color:red}
//     Explainable{explanation: String, text: String}, // ???explanation???{explanation:explanation}
//     FootNote(String), // [^1]
//     InlineMath(String), // $E=mc^2$
//     Reference(String), // [ref: id]
//     URL(String), // [url: link]
//     Bold(String), // **bold text**
//     Italic(String), // *italic text*
//     Strikethrough(String), // ~~strikethrough text~~
//     Underline(String), // __underline text__
//     Superscript(String), // ^^superscript text^^
//     Subscript(String), // ,,subscript text,,
//     MonoSpace(String), // `monospace text`
//     Regular(String), // Regular text
// }


pub fn render_text(input: &str) -> String {
    let mut output = input.to_string();

    // 1. Highlight with color: ==highlighted text=={color:red}
    let re_highlight_color = Regex::new(r"==(?P<text>[^=]+)==\{color:(?P<color>[^}]+)\}").unwrap();
    output = re_highlight_color
        .replace_all(&output, r#"<span class="highlight-$color">$text</span>"#)
        .to_string();

    // 2. Colored text: ===text==={color:red}
    let re_colored_text = Regex::new(r"===(?P<text>[^=]+)===\{color:(?P<color>[^}]+)\}").unwrap();
    output = re_colored_text
        .replace_all(&output, r#"<span class="text-$color">$text</span>"#)
        .to_string();

    // 3. Explainable: ???text???{explanation:explanation}
    let re_explainable = Regex::new(r"\?\?\?(?P<text>.+?)\?\?\?\{explanation:(?P<explanation>[^}]+)\}").unwrap();
    output = re_explainable
        .replace_all(&output, r#"<span class="explainable" data-explanation="$explanation">$text</span>"#)
        .to_string();

    // 4. Footnote: [^1]
    let re_footnote = Regex::new(r"\[\^([^\]]+)\]").unwrap();
    output = re_footnote
        .replace_all(&output, r#"<span class="footnote">[$1]</span>"#)
        .to_string();

    // 5. Inline math: $E=mc^2$
    let re_inline_math = Regex::new(r"\$(?P<text>[^$]+)\$").unwrap();
    output = re_inline_math
        .replace_all(&output, r#"<span class="math">$text</span>"#)
        .to_string();

    // 6. Reference: [ref: id]
    let re_reference = Regex::new(r"\[ref:(?P<text>[^\]]+)\]").unwrap();
    output = re_reference
        .replace_all(&output, r#"<span class="reference">$text</span>"#)
        .to_string();

    // 7. URL: [url: link]
    let re_url = Regex::new(r"\[url:(?P<text>[^\]]+)\]").unwrap();
    output = re_url
        .replace_all(&output, r#"<a href="$text" class="url">$text</a>"#)
        .to_string();

    // 8. Bold: **bold text**
    let re_bold = Regex::new(r"\*\*(?P<text>.+?)\*\*").unwrap();
    output = re_bold
        .replace_all(&output, r#"<span class="bold">$text</span>"#)
        .to_string();

    // 9. Italic: *italic text*
    let re_italic = Regex::new(r"\*(?P<text>.+?)\*").unwrap();
    output = re_italic
        .replace_all(&output, r#"<span class="italic">$text</span>"#)
        .to_string();

    // 10. Strikethrough: ~~strikethrough text~~
    let re_strike = Regex::new(r"~~(?P<text>.+?)~~").unwrap();
    output = re_strike
        .replace_all(&output, r#"<span class="strikethrough">$text</span>"#)
        .to_string();

    // 11. Underline: __underline text__
    let re_underline = Regex::new(r"__(?P<text>.+?)__").unwrap();
    output = re_underline
        .replace_all(&output, r#"<span class="underline">$text</span>"#)
        .to_string();

    // 12. Superscript: ^^superscript text^^
    let re_superscript = Regex::new(r"\^\^(?P<text>.+?)\^\^").unwrap();
    output = re_superscript
        .replace_all(&output, r#"<span class="superscript">$text</span>"#)
        .to_string();

    // 13. Subscript: ,,subscript text,,
    let re_subscript = Regex::new(r",,(?P<text>.+?),,").unwrap();
    output = re_subscript
        .replace_all(&output, r#"<span class="subscript">$text</span>"#)
        .to_string();

    // 14. Monospace: `monospace text`
    let re_monospace = Regex::new(r"`(?P<text>.+?)`").unwrap();
    output = re_monospace
        .replace_all(&output, r#"<span class="monospace">$text</span>"#)
        .to_string();

    // Default: Regular text remains untouched
    output
}

fn format_header(token: &BlockToken) -> String {
    match token {
        BlockToken::Header { level, text } => {
            format!("<h{}>{}</h{}>", level, render_text(text), level)
        }
        _ => "".to_string(),
    }
}

fn format_audio_stamp(token: &BlockToken) -> String {
    match token {
        BlockToken::AudioStamp(text) => {
            format!("<div class=\"audio-stamp\">{}</div>", text)
        }
        _ => "".to_string(),
    }
}

fn format_image(token: &BlockToken) -> String {
    match token {
        BlockToken::Image {
            alt,
            url,
            width,
            reference,
            caption,
        } => {
            format!(
                "<figure>
                    <img src=\"{}\" alt=\"{}\" width=\"{}\">
                    <figcaption>{}</figcaption>
                </figure>",
                url, alt, width, caption
            )
        }
        _ => "".to_string(),
    }
}

fn format_footnote(token: &BlockToken) -> String {
    match token {
        BlockToken::FootNote { id, description } => {
            format!("<sup id=\"fnref:{}\"><a href=\"#fn:{}\" rel=\"footnote\">{}</a></sup>", id, id, render_text(description))
        }
        _ => "".to_string(),
    }
}

fn format_todo_list(token: &BlockToken) -> String {
    match token {
        BlockToken::ToDo { kind, description } => {
            match kind {
                ToDoKind::Task => {
                    format!("<input type=\"checkbox\" disabled>{}", render_text(description))
                }
                ToDoKind::Done => {
                    format!("<input type=\"checkbox\" checked disabled>{}", render_text(description))
                }
            }
        }
        _ => "".to_string(),
    }
}

fn format_horizontal_line(token: &BlockToken) -> String {
    match token {
        BlockToken::HorizontalLine => {
            "<hr>".to_string()
        }
        _ => "".to_string(),
    }
}

fn format_paragraph(token: &BlockToken) -> String {
    match token {
        BlockToken::Paragraph(text) => {
            format!("<p>{}</p>", render_text(text))
        }
        _ => "".to_string(),
    }
}

// format list will be a bit more complex
fn format_list(token: &BlockToken) -> String {
    match token {
        BlockToken::List(text) => {
            format!("<div class=\"list\">{}</div>", text)
        }
        _ => "".to_string(),
    }
}
// format table will be a bit more complex
fn format_table(token: &BlockToken) -> String {
    match token {
        BlockToken::Table { id, content } => {
            format!("<table id=\"{}\">{}</table>", id, content)
        }
        _ => "".to_string(),
    }
}

fn format_callout(token: &BlockToken) -> String {
    match token {
        BlockToken::Callout { kind, info } => {
            let kind_class = match kind {
                CalloutKind::Note => "note",
                CalloutKind::Warning => "warning",
                CalloutKind::Info => "info",
                CalloutKind::Error => "error",
            };
            format!("<div class=\"callout {}\">{}</div>", kind_class, render_text(info))
        }
        _ => "".to_string(),
    }
}

fn format_collapse(token: &BlockToken) -> String {
    match token {
        BlockToken::Collapse(text) => {
            format!("<details><summary>Click to expand</summary>{}</details>", render_text(text))
        }
        _ => "".to_string(),
    }
}

fn format_block_math(token: &BlockToken) -> String {
    match token {
        BlockToken::BlockMath { id, content } => {
            format!("<div id=\"{}\" class=\"block-math\">{}</div>", id, content)
        }
        _ => "".to_string(),
    }
}

fn format_canvas(token: &BlockToken) -> String {
    match token {
        BlockToken::Canvas { id, content } => {
            format!("<div id=\"{}\" class=\"canvas\">{}</div>", id, content)
        }
        _ => "".to_string(),
    }
}

// format image carousel will be a bit more complex
fn format_image_carousel (token: &BlockToken) -> String {
    match token {
        BlockToken::ImageCarousel { id, content } => {
            format!("<div id=\"{}\" class=\"image-carousel\">{}</div>", id, content)
        }
        _ => "".to_string(),
    }
}
// format question will be a bit more complex
fn format_question(token: &BlockToken) -> String {
    match token {
        BlockToken::Question { id, content } => {
            format!("<div id=\"{}\" class=\"question\">{}</div>", id, content)
        }
        _ => "".to_string(),
    }
}

fn format_video(token: &BlockToken) -> String {
    match token {
        BlockToken::Video { id, content } => {
            format!("<div id=\"{}\" class=\"video\">{}</div>", id, content)
        }
        _ => "".to_string(),
    }
}

fn format_quote(token: &BlockToken) -> String {
    match token {
        BlockToken::Quote { id, content } => {
            format!("<div id=\"{}\" class=\"quote\">{}</div>", id, content)
        }
        _ => "".to_string(),
    }
}

fn format_faq(token: &BlockToken) -> String {
    match token {
        BlockToken::FAQ { id, content } => {
            format!("<div id=\"{}\" class=\"faq\">{}</div>", id, content)
        }
        _ => "".to_string(),
    }
}

fn format_code_block(token: &BlockToken) -> String {
    match token {
        BlockToken::CodeBlock { lang, code } => {
            format!("<pre><code class=\"language-{}\">{}</code></pre>", lang, code)
        }
        _ => "".to_string(),
    }
}


pub fn tokens_to_html (tokens: &Vec<BlockToken>) -> String {
    let mut result = String::new();

    for token in tokens {
        let token_str = match token {
            BlockToken::Header { level, text } => format_header(token),
            BlockToken::AudioStamp(text) => format_audio_stamp(token),
            BlockToken::Image { alt, url, width, reference, caption } => format_image(token),
            BlockToken::FootNote { id, description } => format_footnote(token),
            BlockToken::ToDo { kind, description } => format_todo_list(token),
            BlockToken::HorizontalLine => format_horizontal_line(token),
            BlockToken::Paragraph(text) => format_paragraph(token),
            BlockToken::List(text) => format_list(token),
            BlockToken::Table { id, content } => format_table(token),
            BlockToken::Callout { kind, info } => format_callout(token),
            BlockToken::Collapse(text) => format_collapse(token),
            BlockToken::BlockMath { id, content } => format_block_math(token),
            BlockToken::Canvas { id, content } => format_canvas(token),
            BlockToken::ImageCarousel { id, content } => format_image_carousel(token),
            BlockToken::Question { id, content } => format_question(token),
            BlockToken::Video { id, content } => format_video(token),
            BlockToken::Quote { id, content } => format_quote(token),
            BlockToken::FAQ { id, content } => format_faq(token),
            BlockToken::CodeBlock { lang, code } => format_code_block(token),
        };
        result.push_str(&token_str);
        result.push('\n');
    }

    result
}