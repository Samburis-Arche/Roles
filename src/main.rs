#![no_implicit_prelude]
use ::std::*;
use io::Read;
use io::Write;
use iter::Iterator;

enum State {
	Eof,
	Nl,
	Ok,
}

fn skip_line(file: &mut io::Bytes<fs::File>) -> io::Result<State> {
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

fn read_word<F>(file: &mut io::Bytes<fs::File>, mut f: F) -> io::Result<State>
where F: ops::FnMut(u8) {
	match file.next() {
		option::Option::Some(r) => match r? {
			b'\t' => return skip_line(file),
			c => f(c),
		},
		option::Option::None => return result::Result::Ok(State::Eof),
	}
	loop {
		match file.next() {
			option::Option::Some(r) => match r? {
				b'\t' => return result::Result::Ok(State::Ok),
				c => f(c),
			},
			option::Option::None => return result::Result::Ok(State::Eof),
		}
	}
}

fn custom_write(file: &mut fs::File, id: &mut vec::Vec<u8>, emoji: &mut vec::Vec<u8>) -> io::Result<()> {
	file.write_all(id)?;
	file.write_all(emoji)?;
	file.write_all(&[b'\n'])?;
	emoji.clear();
	id.clear();
	return result::Result::Ok(());
}

fn standard_write(file: &mut fs::File, id: &mut vec::Vec<u8>) -> io::Result<()> {
	file.write_all(id)?;
	file.write_all(&[b'\n'])?;
	id.clear();
	return result::Result::Ok(());
}

fn main() -> io::Result<()> {
	let mut in_file = fs::File::open("resources/vaidmenys.tsv")?.bytes();
	let mut out_file = fs::File::create(env::current_exe()?.parent().unwrap().join("out.txt"))?;
	let mut buf_id = vec::Vec::<u8>::with_capacity(100);
	let mut buf_emoji = vec::Vec::<u8>::with_capacity(100);

	match skip_line(&mut in_file)? {
		State::Eof => return result::Result::Ok(()),
		_ => (),
	}

	loop {
		match read_word(&mut in_file, |_| ())? {
			State::Eof => break,
			State::Nl => continue,
			_ => (),
		}
		match read_word(&mut in_file, |c| buf_id.push(c))? {
			State::Eof => break,
			State::Nl => continue,
			_ => (),
		}
		match read_word(&mut in_file, |c| buf_emoji.push(c))? {
			State::Eof => {
				standard_write(&mut out_file, &mut buf_id)?;
				break;
			},
			State::Nl => {
				standard_write(&mut out_file, &mut buf_id)?;
				continue;
			},
			_ => match skip_line(&mut in_file)? {
				State::Eof => {
					custom_write(&mut out_file, &mut buf_id, &mut buf_emoji)?;
					break;
				},
				_ => {
					custom_write(&mut out_file, &mut buf_id, &mut buf_emoji)?;
					continue;
				},
			},
		}
	}

	return result::Result::Ok(());
}
