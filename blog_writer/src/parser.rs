struct ListItem {
    indent: usize,
    description: String,
}

fn parse_line(line: &str) -> ListItem {
    // Count leading spaces
    let mut spaces = 0;
    for c in line.chars() {
        if c == ' ' {
            spaces += 1;
        } else {
            break;
        }
    }

    // Each "2 spaces" is 1 level
    let indent = spaces / 2;

    // Remove those leading spaces
    let trimmed = &line[spaces..];

    // If there's a dash + space, remove them
    let mut description = trimmed.to_string();
    if description.starts_with("- ") {
        description = description[2..].to_string();
    }

    ListItem {
        indent,
        description,
    }
}

fn list_to_html(list: &str) -> String {
    // 1) parse each line into a Vec<ListItem>
    let mut list_items = Vec::new();
    for line in list.lines() {
        // ignore empty lines
        if !line.trim().is_empty() {
            let item = parse_line(line);
            list_items.push(item);
        }
    }

    // 2) build the HTML
    let mut html_string = String::new();

    // Start top-level <ul>
    html_string.push_str("<div class=\"list\">\n");

    let mut past_depth = 0;
    for item in list_items {
        let current_depth = item.indent;

        // If going deeper, open <ul> enough times
        if current_depth > past_depth {
            for _ in past_depth..current_depth {
                html_string.push_str("<div class=\"list\">\n");
            }
        }
        // If going shallower, close <ul> enough times
        else if current_depth < past_depth {
            for _ in current_depth..past_depth {
                html_string.push_str("</div>\n");
            }
        }

        html_string.push_str(&format!(
            "<div class=\"list-item\">{}</div>\n",
            item.description
        ));
        past_depth = current_depth;
    }

    // close out any leftover nesting
    for _ in 0..past_depth {
        html_string.push_str("</div>\n");
    }

    // close the very outer
    html_string.push_str("</div>\n");

    html_string
}

fn table_to_html(raw_string: &str) -> String {
    let mut html_string = String::new();
    let mut title = String::new();

    html_string.push_str("<table class=\"table-class\">\n");
    let mut first_line = true;
    for line in raw_string.lines() {
        // first line is Title: <Something Something>
        if line.trim().starts_with("Title: ") {
            title = line[7..].to_string();
            continue;
        }

        if first_line {
            html_string.push_str("<thead class=\"thead-class\">\n");
            html_string.push_str("<tr>\n");
            for cell in line.split(',') {
                html_string.push_str(&format!("<th scope=\"col\" class=\"th-class\">{}</th>\n", cell));
            }
            html_string.push_str("</tr>\n");
            html_string.push_str("</thead>");
            html_string.push_str("<tbody>");
            first_line = false;
            continue;
        } else {
            html_string.push_str("<tr>\n");
            for cell in line.split(',') {
                html_string.push_str(&format!("<td class=\"td-class\">{}</td>\n", cell));
            }
            html_string.push_str("</tr>\n");
        }
    }

    html_string.push_str("</tbody>");
    html_string.push_str("</table>\n");
    html_string.push_str(&format!("<div class=\"table-title\">{}</div>\n", title));

    html_string
}

fn carousel_to_html(raw_string: &str) -> String {
    let urls = raw_string.lines();
    let mut html_string = String::new();

    for url in urls {
        html_string.push_str(&format!("<img src=\"{}\" class=\"carousel-image\">\n", url.trim()));
    }

    html_string
}


#[derive(Debug, Clone)]
 enum CalloutKind {
    Note,
    Warning,
    Info,
    Error,
}

#[derive(Debug)]
 enum ToDoKind {
    Task,
    Done,
}

 enum BlockToken {
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
 enum AccumulatingType {
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

 fn tokenizer(input: &str) -> Vec<BlockToken> {
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

            // Correct header level calculation
            let level = trimmed.chars().take_while(|c| *c == '#').count();

            // Extract the text, skipping '#' characters and leading whitespace
            let text = trimmed[level..].trim_start();

            tokens.push(BlockToken::Header { level, text: text.to_string() });
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
                    id.trim().to_string() // Extracted ID as a string
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
                    id.trim().to_string() // Extracted ID as a string
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
                    id.trim().to_string() // Extracted ID as a string
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
                    id.trim().to_string() // Extracted ID as a string
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
                    id.trim().to_string() // Extracted ID as a string
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
                    id.trim().to_string() // Extracted ID as a string
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
                    id.trim().to_string() // Extracted ID as a string
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
                    id.trim().to_string() // Extracted ID as a string
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


#[derive(Debug)]
struct TokenMatch {
    replacement: String, // The HTML replacement
    length: usize,       // How many characters of the input were consumed
}

fn try_parse_token(input: &str, i: usize) -> Option<TokenMatch> {
    let remaining: &str = input.get(i..)?;

    // 0. Escape character: \\, \*, \`, \[, \], \=
    if remaining.starts_with("\\") {
        return Some(TokenMatch {
            replacement: remaining.chars().skip(1).take(1).collect(),
            length: 2,
        });
    }
    else if remaining.starts_with("\\*") {
        return Some(TokenMatch {
            replacement: "*".to_string(),
            length: 2,
        });
    }
    else if remaining.starts_with("\\`") {
        return Some(TokenMatch {
            replacement: "`".to_string(),
            length: 2,
        });
    }
    else if remaining.starts_with("\\[") {
        return Some(TokenMatch {
            replacement: "[".to_string(),
            length: 2,
        });
    }
    else if remaining.starts_with("\\]") {
        return Some(TokenMatch {
            replacement: "]".to_string(),
            length: 2,
        });
    }
    else if remaining.starts_with("\\=") {
        return Some(TokenMatch {
            replacement: "=".to_string(),
            length: 2,
        });
    }
    // 1. Colored text: ===text==={color:red}
    else if remaining.starts_with("===") {
        if let Some(end) = remaining.find("===") {
            // end == 0 here, but we actually want the next occurrence:
            let end_delim_pos = remaining[end + 2..].find("===");
            if let Some(second_delim_start) = end_delim_pos {
                // The position of the second '==='
                let second_delim_pos = end + 2 + second_delim_start;
                // We have "=== ... ==="
                let text_content = &remaining[3..second_delim_pos];
                // println!("text_content: {}", text_content);

                // Next, see if there's a {color:...} after that
                let after_markers = &remaining[second_delim_pos + 3..]; // skip "==="
                                                                        // println!("after_markers: {}", after_markers);
                if after_markers.starts_with("{color:") {
                    // parse color
                    if let Some(color_end) = after_markers.find('}') {
                        let color_content = &after_markers[7..color_end]; // skip "{color:"
                        let full_length = 3 + text_content.len() + 3 +  // "===...==="
                                    7 + color_content.len() + 1; // "{color:...}"

                        return Some(TokenMatch {
                            replacement: format!(
                                r#"<span class="color-{}">{}</span>"#,
                                color_content.trim(),
                                text_content.trim()
                            ),
                            length: full_length,
                        });
                    }
                }
            }
        }
    }
    // 2. Highlight with color: ==highlighted text=={color:red}
    else if remaining.starts_with("==") {
        // and then a `{color:...}` block
        if let Some(end) = remaining.find("==") {
            // end == 0 here, but we actually want the next occurrence:
            let end_delim_pos = remaining[end + 2..].find("==");
            if let Some(second_delim_start) = end_delim_pos {
                // The position of the second '=='
                let second_delim_pos = end + 2 + second_delim_start;
                // We have "== ... =="
                let text_content = &remaining[2..second_delim_pos];

                // Next, see if there's a {color:...} after that
                let after_markers = &remaining[second_delim_pos + 2..]; // skip "=="
                if after_markers.starts_with("{color:") {
                    // parse color
                    if let Some(color_end) = after_markers.find('}') {
                        let color_content = &after_markers[7..color_end]; // skip "{color:"
                        let full_length = 2 + text_content.len() + 2 +  // "==...=="
                                          7 + color_content.len() + 1; // "{color:...}"

                        return Some(TokenMatch {
                            replacement: format!(
                                r#"<mark class="{}">{}</mark>"#,
                                color_content.trim(),
                                text_content.trim()
                            ),
                            length: full_length,
                        });
                    }
                }
            }
        }
    }
    // 3. Explainable: ???text???{explanation:...}
    else if remaining.starts_with("???") {
        if let Some(end) = remaining.find("???") {
            // end == 0 here, but we actually want the next occurrence:
            let end_delim_pos = remaining[end + 3..].find("???");
            if let Some(second_delim_start) = end_delim_pos {
                // The position of the second '???'
                let second_delim_pos = end + 3 + second_delim_start;
                // We have "??? ... ???"
                let text_content = &remaining[3..second_delim_pos];

                // Next, see if there's a {explanation:...} after that
                let after_markers = &remaining[second_delim_pos + 3..]; // skip "???"
                if after_markers.starts_with("{explanation:") {
                    // parse explanation
                    if let Some(explanation_end) = after_markers.find('}') {
                        let explanation_content = &after_markers[13..explanation_end]; // skip "{explanation:"
                        let full_length = 3 + text_content.len() + 3 +  // "???...???"
                                          13 + explanation_content.len() + 1; // "{explanation:...}"

                        return Some(TokenMatch {
                            replacement: format!(
                                r#"<span class="explainable" title="{}">{}</span>"#,
                                explanation_content.trim(),
                                text_content.trim()
                            ),
                            length: full_length,
                        });
                    }
                }
            }
        }
    }
    // 7.5 Bold and italic: ***bold and italic***
    else if remaining.starts_with("***") {
        if let Some(end_pos) = remaining[3..].find("***") {
            let bold_italic_text = &remaining[3..3 + end_pos];
            let full_length = 3 + bold_italic_text.len() + 3;
            return Some(TokenMatch {
                replacement: format!(
                    r#"<span class="bold-italic">{}</span>"#,
                    bold_italic_text
                ),
                length: full_length,
            });
        }
    }
    // 8. Bold: **bold text**
    else if remaining.starts_with("**") {
        if let Some(end_pos) = remaining[2..].find("**") {
            let bold_text = &remaining[2..2 + end_pos];
            let full_length = 2 + bold_text.len() + 2;
            return Some(TokenMatch {
                replacement: format!(r#"<span class="bold">{}</span>"#, bold_text),
                length: full_length,
            });
        }
    }
    // 9. Italic: *italic text*
    else if remaining.starts_with('*') {
        if let Some(end_pos) = remaining[1..].find('*') {
            let italic_text = &remaining[1..1 + end_pos];
            let full_length = 1 + italic_text.len() + 1;
            return Some(TokenMatch {
                replacement: format!(r#"<span class="italic">{}</span>"#, italic_text),
                length: full_length,
            });
        }
    }
    // 10. Strikethrough: ~~strikethrough text~~
    else if remaining.starts_with("~~") {
        if let Some(end_pos) = remaining[2..].find("~~") {
            let strike_text = &remaining[2..2 + end_pos];
            let full_length = 2 + strike_text.len() + 2;
            return Some(TokenMatch {
                replacement: format!(r#"<span class="strikethrough">{}</span>"#, strike_text),
                length: full_length,
            });
        }
    }
    // 11. Underline: __underline text__
    else if remaining.starts_with("__") {
        if let Some(end_pos) = remaining[2..].find("__") {
            let underline_text = &remaining[2..2 + end_pos];
            let full_length = 2 + underline_text.len() + 2;
            return Some(TokenMatch {
                replacement: format!(r#"<span class="underline">{}</span>"#, underline_text),
                length: full_length,
            });
        }
    }
    // 12. Superscript: ^^superscript text^^
    else if remaining.starts_with("^^") {
        if let Some(end_pos) = remaining[2..].find("^^") {
            let superscript_text = &remaining[2..2 + end_pos];
            let full_length = 2 + superscript_text.len() + 2;
            return Some(TokenMatch {
                replacement: format!(r#"<sup>{}</sup>"#, superscript_text),
                length: full_length,
            });
        }
    }
    // 13. Subscript: ,,subscript text,,
    else if remaining.starts_with(",,") {
        if let Some(end_pos) = remaining[2..].find(",,") {
            let subscript_text = &remaining[2..2 + end_pos];
            let full_length = 2 + subscript_text.len() + 2;
            return Some(TokenMatch {
                replacement: format!(r#"<sub>{}</sub>"#, subscript_text),
                length: full_length,
            });
        }
    }
    // 14. Monospace: `monospace text`
    else if remaining.starts_with('`') {
        if let Some(end_pos) = remaining[1..].find('`') {
            let mono_text = &remaining[1..1 + end_pos];
            let full_length = 1 + mono_text.len() + 1;
            return Some(TokenMatch {
                replacement: format!(r#"<span class="monospace">{}</span>"#, mono_text),
                length: full_length,
            });
        }
    }
    // 15. Reference: [ref: text]
    else if remaining.starts_with("[ref: ") {
        if let Some(end_pos) = remaining[6..].find(']') {
            let ref_text = &remaining[6..6 + end_pos];
            let full_length = 6 + ref_text.len() + 1;
            return Some(TokenMatch {
                replacement: format!(r#"<span class="reference">{}</span>"#, ref_text),
                length: full_length,
            });
        }
    }
    // 16. URL: [text](url)
    else if remaining.starts_with('[') {
        if let Some(end_bracket) = remaining.find(']') {
            let text_content = &remaining[1..end_bracket]; // Extract text between '[' and ']'
            let after_bracket = &remaining[end_bracket + 1..]; // Move past ']'

            if after_bracket.starts_with('(') {
                if let Some(end_paren) = after_bracket.find(')') {
                    let url_content = &after_bracket[1..end_paren]; // Extract URL between '(' and ')'
                    let full_length = 1 + text_content.len() + 1 + 1 + url_content.len() + 1; // Calculate total length

                    return Some(TokenMatch {
                        replacement: format!(
                            r#"<a href="{}">{}</a>"#,
                            url_content.trim(),
                            text_content.trim()
                        ),
                        length: full_length,
                    });
                }
            }
        }
    }
    None
}

 fn render_text(input: &str) -> String {
    let mut output = String::new();
    let mut i = 0;
    while i < input.len() {
        if let Some(token_match) = try_parse_token(input, i) {
            // If we recognized a special token
            output.push_str(&token_match.replacement);
            i += token_match.length;
        } else {
            // No special token recognized, copy a single character
            output.push(input.as_bytes()[i] as char);
            i += 1;
        }
    }
    output
}

fn format_header(token: &BlockToken) -> String {
    match token {
        BlockToken::Header { level, text } => {
            format!("<h{}>{}</h{}>", level + 2, render_text(text), level + 2)
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
                "<div class=\"figure\" id=\"{}\">
                    <div class=\"figure-caption\">
                        <span class=\"material-symbols-outlined\">format_quote</span>
                        <div class=\"figure-caption-text\">{}</div>
                    </div>
                    <img src=\"{}\" alt=\"{}\" width=\"{}\" aspect-ratio=\"auto\">
                </div>",
                reference, caption, url, alt, width
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
                    format!("<div class=\"todo-item\"><input type=\"checkbox\" disabled>{}</div>", render_text(description))
                }
                ToDoKind::Done => {
                    format!("<div class=\"todo-item\"><input type=\"checkbox\" checked disabled>{}</div>", render_text(description))
                }
            }
        }
        _ => "".to_string(),
    }
}

fn format_horizontal_line(token: &BlockToken) -> String {
    match token {
        BlockToken::HorizontalLine => {
            "<hr class=\"custom-hr\">".to_string()
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

fn format_list(token: &BlockToken) -> String {
    match token {
        BlockToken::List(raw_list) => {
            format!("<div class=\"list-container\">{}</div>", list_to_html(&render_text(raw_list)))
        }
        _ => "".to_string(),
    }
}

fn format_table(token: &BlockToken) -> String {
    match token {
        BlockToken::Table { id, content } => {
            format!("<div id=\"{}\" class=\"table-container\">{}</div>", id, table_to_html(&content))
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
            let icon_name = match kind {
                CalloutKind::Note => "note_stack",
                CalloutKind::Warning => "priority_high",
                CalloutKind::Info => "lightbulb_2",
                CalloutKind::Error => "error",
            };
            format!("<div class=\"callout-container\"><div class=\"callout-logo-{}\"><span class=\"material-symbols-outlined\">{}</span></div><div class=\"vertical-line\"></div><div class=\"callout-content\">{}</div></div>", kind_class, icon_name, render_text(info))
        }
        _ => "".to_string(),
    }
}

fn format_collapse(token: &BlockToken) -> String {
    match token {
        BlockToken::Collapse(text) => {
            format!("<details class=\"collapse-container\"><summary class=\"collapse-summary\">Click to expand</summary><span>{}</span></details>", render_text(text))
        }
        _ => "".to_string(),
    }
}

fn format_block_math(token: &BlockToken) -> String {
    match token {
        BlockToken::BlockMath { id, content } => {
            format!("<div id=\"{}\" class=\"block-math\">\\[\n{}\n\\]</div>", id, content)
        }
        _ => "".to_string(),
    }
}

fn format_canvas(token: &BlockToken) -> String {
    match token {
        BlockToken::Canvas { id, content } => {
            format!(
                r#"
                <canvas id="{id}" class="canvas-container" width="500px" height="500px"></canvas>
                <script>
                    const canvas = document.getElementById("{id}");
                    {content}
                </script>
                "#,
                id = id,
                content = content
            )
        }
        _ => "".to_string(),
    }
}

fn format_image_carousel (token: &BlockToken) -> String {
    match token {
        BlockToken::ImageCarousel { id, content } => {
            format!("<div id=\"{}\" class=\"image-carousel\">{}</div>", id, carousel_to_html(content))
        }
        _ => "".to_string(),
    }
}

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

 fn tokens_to_html (tokens: &Vec<BlockToken>) -> String {
    let mut result = String::new();

    for token in tokens {
        let token_str = match token {
            BlockToken::Header { .. } => format_header(token),
            BlockToken::AudioStamp(_) => format_audio_stamp(token),
            BlockToken::Image { .. } => format_image(token),
            BlockToken::FootNote { .. } => format_footnote(token),
            BlockToken::ToDo { .. } => format_todo_list(token),
            BlockToken::HorizontalLine => format_horizontal_line(token),
            BlockToken::Paragraph(_) => format_paragraph(token),
            BlockToken::List(_) => format_list(token),
            BlockToken::Table { .. } => format_table(token),
            BlockToken::Callout { .. } => format_callout(token),
            BlockToken::Collapse(_) => format_collapse(token),
            BlockToken::BlockMath { .. } => format_block_math(token),
            BlockToken::Canvas { .. } => format_canvas(token),
            BlockToken::ImageCarousel { .. } => format_image_carousel(token),
            BlockToken::Question { .. } => format_question(token),
            BlockToken::Video { .. } => format_video(token),
            BlockToken::Quote { .. } => format_quote(token),
            BlockToken::FAQ { .. } => format_faq(token),
            BlockToken::CodeBlock { .. } => format_code_block(token),
        };
        result.push_str(&token_str);
        result.push('\n');
    }

    result
}

use std::fs;
pub fn markdown_to_html(input: &str, background_image_url: &str, title: &str, author: &str, created_on: &str) -> String {
    // open file css.txt and read its contents
    let css_path = "css.txt";
    // read the contents of the file, if not found, return an empty string
    let css_content = fs::read_to_string(css_path).unwrap_or("".to_string());


    let html_start = format!(r#"
<!DOCTYPE html>
<html lang='en'>

<head>
  <meta charset='UTF-8'>
  <meta name='viewport' content='width=device-width, initial-scale=1.0'>
  <title>Tailwind Project</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link
    href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap"
    rel="stylesheet">
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined&display=swap">
  <script>
    MathJax = {{
      tex: {{
        inlineMath: [['$', '$']]
    }},
      svg: {{
        fontCache: 'global'
      }}
    }};
  </script>
  <script type="text/javascript" id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js">
  </script>
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css">
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js"></script>

  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/bash.min.js"
    integrity="sha512-nQ9BQEzuov+Ry6EIH8ve7VKKdOG91Ix3SAQcFmOiBR5qG8sJONrph1InWTJOGjfP5QkSTSy4VnkEsPMoFYRsUQ=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/c.min.js"
    integrity="sha512-HheXunBWZVAec/gPxpzgxGZmajdCQWVBxMba1VaUAADs8g7GmlB8k8346ihWgGtyrjfr9Qjmk7+IFHrjj2OaJQ=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/cpp.min.js"
    integrity="sha512-i2YY1iYKnV+oOmxBA9Jd4YFwM8E07ySVuVEEhPWQ/gSVHVvzWhHbvK1REz1KuZL4amejLDjslRvMIo8ORiTu6Q=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/csharp.min.js"
    integrity="sha512-E508cDPDWSFwXbo0jUpAxAuDtuKyg3qzALy/oyrTBcP07/Xz79ajxXl4LzoNAZw+hr4NAnjPhAdwN0qtmJ9ukg=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/css.min.js"
    integrity="sha512-fQhadp2Av4sRhTYd9TEggQW4NrosThwDUXIMLin90uepvmCZG0mRTdPVtHujrXD22qMTtdQlrAEdvWocezI8ow=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/haskell.min.js"
    integrity="sha512-wV1s4ylNcflirsC0Ug9dDahOxjj/JSQheHv0loB9Q3bv7G0TYLduOWWmhz2MjMKRO6+LS8AgeuhBB8Gny4pdFQ=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/java.min.js"
    integrity="sha512-DTx6faal3nhEB55v+yn8UnXCxaxCdMR6gBZ0zzXhGD2qvtgf6xbW5iA7G4CUn78R9PoPzTelR7xIQwGNUJAv2w=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/javascript.min.js"
    integrity="sha512-yfJUrNGEC39mHLjZ37CZG69Ij9Vnan7NHxXVuuBxafgfk4F+n7j/NhNWtyhKGTYEgWfgUqzPYMZJZY1HIsPCbQ=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/json.min.js"
    integrity="sha512-f2/ljYb/tG4fTHu6672tyNdoyhTIpt4N1bGrBE8ZjwIgrjDCd+rljLpWCZ2Vym9PBWQy2Tl9O22Pp2rMOMvH4g=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/latex.min.js"
    integrity="sha512-4h3uWtv1iEA4sgWqZ1is70GnmESsRzCRaGTyeajS/8Zq0CGWVCCRTrpEw5mJ57vF9LIfwNwCMNeV6IjGytrhrA=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/makefile.min.js"
    integrity="sha512-lLnMPGLNcEgvwt6f291Bp+WqF3HRxv+8q1t6PA/XTwZH/D1XLB2tUayRAjfAp6AAUrrieJnXQCUtGZtcMhP6ag=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/markdown.min.js"
    integrity="sha512-j4HfSOGvYNRQiq8wvftDi9oVmhSqJuh2dALD+Btn53looHt+e1aaNX9gcQsnUAi42694fFhmmU4fx8nzs6VN7Q=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/nginx.min.js"
    integrity="sha512-kvYzE8uXJNOxB79FbRApSqfjbo7ngD2ulfLeAS5yrDpVL2vpgsw8FhM/N77/xstE183jysYBacXAOrZtgQWW/A=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/python.min.js"
    integrity="sha512-/uCTceIDOniHf+VUKbCnP/x6GQSRrm4GwUtQYMgKa9yIZPGzlR04flSsD+2or7bPn44VY9inIHI4cwNCcZmJDw=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/rust.min.js"
    integrity="sha512-uSwY2GKSvbwBXA67+Hk3onpjVKHLMc734wgIclGBo3XUVnaksT0Mp3qRjmzVMAsAUrk8ZxbSxGsIQ8I0XkY3Gg=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/sql.min.js"
    integrity="sha512-RzK8J6sL3xE7eyppcm7hjCq6fpW1kDlnKPO5jX+Z++/ChYXqsq5AoFq0z78g3jf/1DV5wMBZukqIyb+0xPkCVw=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/verilog.min.js"
    integrity="sha512-bFy4K4cSBOA7MNrKZThhLeok+EAwwR+R3J0bFVKczK0h9MAVEPVSYMWoIGDs3yOJ2WxLJUEeJ1dkpM8vdasxqw=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/vhdl.min.js"
    integrity="sha512-sVEWS+8DbPq026tWk3RggYqiRKU9pwpelEdUgNnJ7ow3Wb99nn5E7Nr8EcjPFjwAK6z/o/ebqs56Z6/0d5Iw7Q=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/x86asm.min.js"
    integrity="sha512-gLk8o5O27h5BbPkdiO0RMiAvU0d8U+S/ft7X5Yh2rpg9XdQ0wBgkjSQ7qz4R6sO9A7AzeLQJ5pKpXGs4jks05A=="
    crossorigin="anonymous" referrerpolicy="no-referrer"></script>

  <script>hljs.highlightAll();</script>
  <style>
    :root {{
          --background-image: url('{}');
      }}
      body {{
          background-image: var(--background-image);
          background-size: cover;
          background-position: center;
          background-repeat: no-repeat;
          background-attachment: fixed;
      }}
    .material-symbols-outlined {{
      font-variation-settings:
      'FILL' 0,
      'wght' 400,
      'GRAD' 0,
      'opsz' 24;
    }}
  </style>
  <script>
function getContrastiveColorWithMode(imageUrl, callback) {{
  const img = new Image();
  img.crossOrigin = "Anonymous"; // Allow cross-origin images
  img.src = imageUrl;

  img.onload = function () {{
      // Create a canvas to process the image
      const canvas = document.createElement("canvas");
      canvas.width = img.width;
      canvas.height = img.height;

      const ctx = canvas.getContext("2d");
      ctx.drawImage(img, 0, 0, img.width, img.height);

      // Extract image data
      const imageData = ctx.getImageData(0, 0, img.width, img.height);
      const data = imageData.data;

      let totalR = 0, totalG = 0, totalB = 0;

      // Loop through pixels to calculate the total RGB values
      for (let i = 0; i < data.length; i += 4) {{
          totalR += data[i];
          totalG += data[i + 1];
          totalB += data[i + 2];
      }}

      // Calculate average RGB
      const pixelCount = data.length / 4;
      const avgR = totalR / pixelCount;
      const avgG = totalG / pixelCount;
      const avgB = totalB / pixelCount;

      // Calculate average luminance of the image
      const calculateLuminance = (r, g, b) => {{
          const normalize = (val) => (val / 255 <= 0.03928 ? val / 255 / 12.92 : ((val / 255 + 0.055) / 1.055) ** 2.4);
          return 0.2126 * normalize(r) + 0.7152 * normalize(g) + 0.0722 * normalize(b);
      }};
      const backgroundLuminance = calculateLuminance(avgR, avgG, avgB);

      // Decide light or dark mode based on luminance
      const isDarkMode = backgroundLuminance > 0.5; // Luminance > 0.5 -> light background

      // Adjust contrastive color in HSL
      const rgbToHsl = (r, g, b) => {{
          r /= 255;
          g /= 255;
          b /= 255;

          const max = Math.max(r, g, b);
          const min = Math.min(r, g, b);
          const delta = max - min;

          let h = 0, s = 0, l = (max + min) / 2;

          if (delta !== 0) {{
              s = l > 0.5 ? delta / (2 - max - min) : delta / (max + min);

              switch (max) {{
                  case r: h = ((g - b) / delta + (g < b ? 6 : 0)) * 60; break;
                  case g: h = ((b - r) / delta + 2) * 60; break;
                  case b: h = ((r - g) / delta + 4) * 60; break;
              }}
          }}

          return {{ h: Math.round(h), s: Math.round(s * 100), l: Math.round(l * 100) }};
      }};

      const hsl = rgbToHsl(avgR, avgG, avgB);

      // Rotate hue for contrasting color
      const contrastHue = (hsl.h + 180) % 360;
      const contrastSaturation = Math.min(hsl.s + 30, 100);
      const contrastLightness = isDarkMode ? Math.min(hsl.l + 50, 85) : Math.max(hsl.l - 50, 15);

      const hslToRgb = (h, s, l) => {{
          s /= 100;
          l /= 100;

          const c = (1 - Math.abs(2 * l - 1)) * s;
          const x = c * (1 - Math.abs((h / 60) % 2 - 1));
          const m = l - c / 2;

          let r = 0, g = 0, b = 0;

          if (h >= 0 && h < 60) {{ r = c; g = x; b = 0; }}
          else if (h >= 60 && h < 120) {{ r = x; g = c; b = 0; }}
          else if (h >= 120 && h < 180) {{ r = 0; g = c; b = x; }}
          else if (h >= 180 && h < 240) {{ r = 0; g = x; b = c; }}
          else if (h >= 240 && h < 300) {{ r = x; g = 0; b = c; }}
          else if (h >= 300 && h < 360) {{ r = c; g = 0; b = x; }}

          r = Math.round((r + m) * 255);
          g = Math.round((g + m) * 255);
          b = Math.round((b + m) * 255);

          return {{ r, g, b }};
      }};

      const contrastColorRgb = hslToRgb(contrastHue, contrastSaturation, contrastLightness);

      const contrastiveColor = `rgb(${{contrastColorRgb.r}}, ${{contrastColorRgb.g}}, ${{contrastColorRgb.b}})`;
      callback(contrastiveColor, isDarkMode);
  }};
}}

function applyContrastiveTextColor(imageUrl) {{
  getContrastiveColorWithMode(imageUrl, (contrastiveColor, isDarkMode) => {{
      const root = document.documentElement;

      // Apply the chosen text color and mode
      root.style.setProperty("--color-text-light", contrastiveColor);
      root.style.setProperty("--color-text-dark", contrastiveColor);
      root.style.setProperty("--theme-mode", isDarkMode ? "light" : "dark");

      if (isDarkMode) {{
          document.body.classList.add("light-mode");
          document.body.classList.remove("dark-mode");
      }} else {{
          document.body.classList.add("dark-mode");
          document.body.classList.remove("light-mode");
      }}
  }});
}}

// Call the function with your image URL
applyContrastiveTextColor("{}");
  </script>
  <style>{}</style>
</head> 
<body>
  <div class='glass'>
    <div class='content'>
"#, background_image_url, background_image_url, css_content);
    let html_extra = format!(r#"
<div class="flex flex-col">
<div class="flex flex-row gap-6">
    <div><a href="/"><span class="material-symbols-outlined" style="font-size: 32px;">home</span></a></div>
</div>
<h1>{}</h1>
  <h2>Author: {}</h2>
  <h2>Created In: {}</h2>
</div>"#, title, author, created_on);
    let html_end = r#"
</div></div></body></html>
    "#;

    let tokens = tokenizer(input);
    let html_content = tokens_to_html(&tokens);
    let full_html = format!("{}{}{}{}", html_start, html_extra, html_content, html_end);
    full_html
}