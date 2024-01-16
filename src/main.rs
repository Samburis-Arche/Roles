#![no_implicit_prelude]
use ::std::*;
use io::Read;
use io::Write;
use iter::Iterator;

enum State {
	Eof,
	Nl,
	TabNl,
	Ok,
}

fn skip_line(file: &mut io::Bytes<fs::File>) -> io::Result<State> {
	loop {
		match file.next() {
			option::Option::Some(r) => match r? {
				b'\n' => return result::Result::Ok(State::TabNl),
				b'\t' | b'\r' => continue,
				_ => break,
			},
			option::Option::None => return result::Result::Ok(State::Eof),
		}
	}
	loop {
		match file.next() {
			option::Option::Some(r) => match r? {
				b'\n' => return result::Result::Ok(State::Nl),
				_ => continue,
			},
			option::Option::None => return result::Result::Ok(State::Eof),
		}
	}
}

fn read_word(file: &mut io::Bytes<fs::File>, v: &mut vec::Vec<u8>) -> io::Result<State> {
	match file.next() {
		option::Option::Some(r) => match r? {
			b'\t' => return skip_line(file),
			c => v.push(c),
		},
		option::Option::None => return result::Result::Ok(State::Eof),
	}
	loop {
		match file.next() {
			option::Option::Some(r) => match r? {
				b'\t' => return result::Result::Ok(State::Ok),
				c => v.push(c),
			},
			option::Option::None => return result::Result::Ok(State::Eof),
		}
	}
}

fn main() -> io::Result<()> {
	println!("{:?}", env::current_dir()?);

	let mut in_file = fs::File::open("resources/vaidmenys.tsv")?.bytes();
	let mut out_file = fs::File::create(env::current_exe()?.parent().unwrap().join("out.txt"))?;

	let mut buf_name = vec::Vec::<u8>::with_capacity(100);
	let mut buf_id = vec::Vec::<u8>::with_capacity(100);
	let mut buf_emoji = vec::Vec::<u8>::with_capacity(100);

	const EMOJIS: [char; 20] = ['🇦', '🇧', '🇨', '🇩', '🇪', '🇫', '🇬', '🇭', '🇮', '🇯', '🇰', '🇱', '🇲', '🇳', '🇴', '🇵', '🇶', '🇷', '🇸', '🇹'];
	let mut emojis_iter = EMOJIS.iter();

	match skip_line(&mut in_file)? {
		State::Eof => return result::Result::Ok(()),
		_ => (),
	}

	loop {
		match read_word(&mut in_file, &mut buf_name)? {
			State::Eof => break,
			State::TabNl | State::Nl => continue,
			State::Ok => (),
		}
		match read_word(&mut in_file, &mut buf_id)? {
			State::Eof => break,
			State::Nl => {
				buf_name.clear();
				continue;
			},
			State::TabNl => {
				writeln!(out_file)?;
				unsafe { writeln!(out_file, "• {}", str::from_utf8_unchecked(&buf_name))? }
				buf_name.clear();
				continue;
			},
			State::Ok => buf_name.clear(),
		}
		let emoji = *match emojis_iter.next() {
			option::Option::Some(e) => e,
			option::Option::None => {
				writeln!(out_file, "\n\n")?;
				emojis_iter = EMOJIS.iter();
				unsafe { emojis_iter.next().unwrap_unchecked() }
			},
		};
		match read_word(&mut in_file, &mut buf_emoji)? {
			State::Ok => {
				unsafe { writeln!(out_file, "> {} `-` <@&{}>", str::from_utf8_unchecked(&buf_emoji), str::from_utf8_unchecked(&buf_id))? }
				buf_emoji.clear();
				buf_id.clear();

				match skip_line(&mut in_file)? {
					State::Eof => break,
					_ => continue,
				}
			},
			state => {
				unsafe { writeln!(out_file, "> {} `-` <@&{}>", emoji, str::from_utf8_unchecked(&buf_id))? }
				buf_id.clear();

				match state {
					State::Eof => break,
					State::TabNl | State::Nl => continue,
					State::Ok => unsafe { hint::unreachable_unchecked() },
				}
			},
		}
	}

	return result::Result::Ok(());
}
