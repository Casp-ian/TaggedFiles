use std::cmp;

use super::types::{ChildTag, Tag, TagFilter};

// for now we do not allow child of child tags
pub fn parse_tags(tags: &Vec<String>) -> Result<TagFilter, String> {
    let mut allowed_tags: Vec<String> = vec![];
    let mut denied_tags: Vec<String> = vec![];
    let mut allowed_child_tags: Vec<(String, String)> = vec![];
    let mut denied_child_tags: Vec<(String, String)> = vec![];

    // realising now that this could have been implemented with 4 regexes...
    // i will swap the implementation when its time to touch this file again
    // maybe regexes will be less performant that the current implementation

    for tag_strings in tags {
        do_tag_parsing(
            &mut allowed_tags,
            &mut denied_tags,
            &mut allowed_child_tags,
            &mut denied_child_tags,
            tag_strings,
        )?;
    }

    return Ok(TagFilter {
        allowed_tags,
        denied_tags,
        allowed_child_tags,
        denied_child_tags,
    });
}

fn do_tag_parsing(
    allowed_tags: &mut Vec<String>,
    denied_tags: &mut Vec<String>,
    allowed_child_tags: &mut Vec<(String, String)>,
    denied_child_tags: &mut Vec<(String, String)>,
    tag_string: &String,
) -> Result<(), String> {
    // walk till slash and do children, if end of line its done

    let slash_location = tag_string.find("/");
    // validate that not more than two "/"?

    let parent_tag_string: &str;
    if let Some(index) = slash_location {
        parent_tag_string = &tag_string[..index];

        do_child_tags_recursive(
            allowed_child_tags,
            denied_child_tags,
            tag_string[index + 1..].to_owned(),
            parent_tag_string.to_string().clone(),
        )?;
    } else {
        parent_tag_string = tag_string;
    }

    // TODO extract negative positive thing to function
    if parent_tag_string.starts_with('=') {
        if slash_location.is_some() {
            // TODO think of a maybe better solution than just an error
            return Err("having a denied parent tag with child tags to check for is kind of useless/contradictory".to_owned());
        }
        denied_tags.push(parent_tag_string[1..].to_owned())
    } else if parent_tag_string.starts_with('+') {
        allowed_tags.push(parent_tag_string[1..].to_owned())
    } else {
        allowed_tags.push(parent_tag_string.to_owned())
    }

    return Ok(());
}

fn do_child_tags_recursive(
    allowed_child_tags: &mut Vec<(String, String)>,
    denied_child_tags: &mut Vec<(String, String)>,
    children_tag_string: String,
    parent_tag_string: String,
) -> Result<(), String> {
    if children_tag_string.is_empty() {
        return Err("Expected another child tag".to_owned());
    }

    let positive;
    let new_children_string;

    if children_tag_string.starts_with('=') {
        positive = false;
        // cut of first char
        new_children_string = children_tag_string[1..].to_owned();
    } else if children_tag_string.starts_with('+') {
        positive = true;
        // cut of first char
        new_children_string = children_tag_string[1..].to_owned();
    } else {
        positive = true;
        new_children_string = children_tag_string;
    }

    let plus_location = new_children_string.find('+');
    let minus_location = new_children_string.find('=');

    let next_location: Option<usize> = match (plus_location, minus_location) {
        (None, None) => None,
        (plus_index, None) => plus_index,
        (None, minus_index) => minus_index,
        (plus_index, minus_index) => cmp::min(plus_index, minus_index),
    };

    if let Some(index) = next_location {
        if positive {
            allowed_child_tags.push((
                parent_tag_string.clone(),
                new_children_string[..index].to_owned(),
            ));
        } else {
            denied_child_tags.push((
                parent_tag_string.clone(),
                new_children_string[..index].to_owned(),
            ));
        }

        do_child_tags_recursive(
            allowed_child_tags,
            denied_child_tags,
            new_children_string[index..].to_owned(),
            parent_tag_string,
        )?;
    } else {
        // this is where we stop recursing
        if positive {
            allowed_child_tags.push((parent_tag_string, new_children_string));
        } else {
            denied_child_tags.push((parent_tag_string, new_children_string));
        }
    }

    return Ok(());
}
