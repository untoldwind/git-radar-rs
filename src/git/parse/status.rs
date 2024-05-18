use std::error::Error;

use nom::{
    branch::alt,
    character::complete::{anychar, line_ending, not_line_ending, one_of},
    combinator::iterator,
    IResult,
};

use crate::git::types::{GitFileState, GitLocalRepoChanges};

pub fn git_parse_status(input: &[u8]) -> Result<GitLocalRepoChanges, Box<dyn Error>> {
    Ok(parse_local(input)? + parse_index(input)?)
}

fn parse_local(input: &[u8]) -> Result<GitLocalRepoChanges, Box<dyn Error>> {
    let mut iter = iterator(input, get_local_lines);
    let changes = iter.collect::<GitLocalRepoChanges>();
    iter.finish().map_err(|err| format!("{}", err))?;

    Ok(changes)
}

fn parse_index(input: &[u8]) -> Result<GitLocalRepoChanges, Box<dyn Error>> {
    let mut iter = iterator(input, get_index_lines);
    let changes = iter.collect::<GitLocalRepoChanges>();
    iter.finish().map_err(|err| format!("{}", err))?;

    Ok(changes)
}

fn get_local_lines(input: &[u8]) -> IResult<&[u8], GitFileState> {
    let (input, state) = local_file_state(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, state))
}

fn get_index_lines(input: &[u8]) -> IResult<&[u8], GitFileState> {
    let (input, state) = index_file_state(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, state))
}

fn index_file_state(input: &[u8]) -> IResult<&[u8], GitFileState> {
    let (input, state) = alt((
        conflic_state1,
        conflic_state2,
        conflic_state3,
        renamed_state,
        index_mod_state,
        index_add_state,
        index_del_state,
        skip_line,
    ))(input)?;
    let (input, _) = not_line_ending(input)?;
    Ok((input, state))
}

fn local_file_state(input: &[u8]) -> IResult<&[u8], GitFileState> {
    let (input, state) =
        alt((local_mod_state, local_add_state, local_del_state, skip_line))(input)?;
    let (input, _) = not_line_ending(input)?;
    Ok((input, state))
}

fn skip_line(input: &[u8]) -> IResult<&[u8], GitFileState> {
    let (input, _) = anychar(input)?;
    Ok((input, GitFileState::Skip))
}

macro_rules! tow_chars_parser {
    ($name: ident, $first: literal, $second: literal, $state: ident) => {
        fn $name(input: &[u8]) -> IResult<&[u8], GitFileState> {
            let (input, _) = one_of($first)(input)?;
            let (input, _) = one_of($second)(input)?;
            Ok((input, GitFileState::$state))
        }
    };
}

tow_chars_parser!(conflic_state1, "D", "DU", Conflict);
tow_chars_parser!(conflic_state2, "A", "AU", Conflict);
tow_chars_parser!(conflic_state3, "U", "AUD", Conflict);
tow_chars_parser!(local_mod_state, "MARC ", "M", LocalMod);
tow_chars_parser!(local_add_state, "?", "?", LocalAdd);
tow_chars_parser!(local_del_state, "MARC ", "D", LocalDel);
tow_chars_parser!(index_mod_state, "M", "DM ", IndexMod);
tow_chars_parser!(index_add_state, "A", "DM ", IndexAdd);
tow_chars_parser!(index_del_state, "D", "M ", IndexDel);
tow_chars_parser!(renamed_state, "R", "DM ", Renamed);
