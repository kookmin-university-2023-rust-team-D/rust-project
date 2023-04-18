let  lines = open_file(filename);

let mut lines = match  lines{
    Ok(lines) => lines,
    Err(err) => panic!("파일을 여는 도중에 문제가 생겼습니다! {:?}", err),
};

let line = lines.next().expect("파일을 읽을 수 없습니다!");
let mut status = match line {
    Ok(val) => val.split_whitespace().map(|s| s),
    Err(_err) => panic!("파일을 읽을 수 없습니다!"),
};