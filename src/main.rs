use std::{
    // fs,
    fs::File,
    io::{prelude::*, BufReader, copy},
    net::{TcpListener, TcpStream},
    path::Path,
};
// extern crate memchr;
// use memchr::memchr;
// use substring::Substring;
// use std::slice;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = vec![0u8; 102400000];
    stream.read_to_end(&mut buffer).unwrap();
    // let first_num_bytes = stream.read(&mut buffer).unwrap();
    // let mut total_bytes = first_num_bytes;
    // loop{
    //     let num_bytes = stream.read(&mut buffer).unwrap();
    //     total_bytes += num_bytes;
    //     // println!("{:#?}", num_bytes);
    //     if num_bytes < first_num_bytes{break;}
    // }
    // println!("loop broken");
    // buffer.truncate(total_bytes);
    // buffer.shrink_to_fit();
    println!("{:#?}", buffer);
    println!("req:\n{}", String::from_utf8_lossy(&buffer[..]));
    
    let binding = String::from_utf8_lossy(&buffer[..]);
    // println!("{:#?}", binding);
    let line_iterator: Vec<&str> = binding.split("\n").collect();
    // line_iterator.asd();
    // for s in split{println!("{:#?}", s);}
    let line_iterator_string = line_iterator[0].to_string();
    let space_iterator: Vec<&str> = line_iterator_string.split(" ").collect();
    // let mut space_iterator = &line_iterator.next().unwrap().split(" ");
    // println!("{:#?}", space_iterator);
    let method = space_iterator[0];
    // println!("{:#?}", method);
    // let method = space_iterator.next().unwrap();
    // println!("{:#?}", String::from_utf8_lossy(&buffer[..]));

    
    if method == "GET"{
    //     let filename = space_iterator.next();
    //     let filename = filename.unwrap();
        let filename = space_iterator[1];
        println!("{:#?}", filename);
        
        if filename == "/"{
            let filename = "index.html";
            let b = Path::new(filename).exists();
            println!("{}", b);
            let status_line = "HTTP/1.1 200 OK";
            let mut reader = BufReader::new(File::open(filename).unwrap());
            let length = File::open(filename).unwrap().metadata().unwrap().len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
            
            stream.write_all(response.as_bytes()).unwrap();
            let result = copy(&mut reader, &mut stream);
            println!("{:#?}", result);
        } else {
            let filename = &filename.trim_start_matches('/');
            println!("{:#?}", filename);
            if Path::new(filename).exists(){
                let status_line = "HTTP/1.1 200 OK";
                let mut reader = BufReader::new(File::open(filename).unwrap());
                let length = File::open(filename).unwrap().metadata().unwrap().len();

                let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
                
                stream.write_all(response.as_bytes()).unwrap();
                let result = copy(&mut reader, &mut stream);
                println!("{:#?}", result);
            } else if !Path::new(filename).exists(){
                // let filename = filename.push_str(".html");
                let filename = filename.to_string() + ".html";
                // filename.asd();
                let filename: &str = &filename;
                println!("{:#?}", filename);
                if Path::new(filename).exists(){
                    let status_line = "HTTP/1.1 200 OK";
                    let mut reader = BufReader::new(File::open(filename).unwrap());
                    let length = File::open(filename).unwrap().metadata().unwrap().len();

                    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
                    
                    stream.write_all(response.as_bytes()).unwrap();
                    let result = copy(&mut reader, &mut stream);
                    println!("{:#?}", result);
                } else {
                    let filename = "404.html";
                    if Path::new(filename).exists(){
                        let status_line = "HTTP/1.1 200 OK";
                        let mut reader = BufReader::new(File::open(filename).unwrap());
                        let length = File::open(filename).unwrap().metadata().unwrap().len();

                        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
                        
                        stream.write_all(response.as_bytes()).unwrap();
                        let result = copy(&mut reader, &mut stream);
                        println!("{:#?}", result);}
                }
            }
        }
    } else if method == "POST"{
        println!("POST");
        // println!("{:#?}", line_iterator);
        // let mut boundary_line = "none";
        // for line in line_iterator_string.lines(){
        //     if line.contains("Content-Type"){
        //         boundary_line = line;
        //         break;
        //     }
        //     else{
        //         boundary_line = "none";
        //         break;
        //     }
        // }
        let mut i = 0;
        let mut boundary_line;
        loop{
            boundary_line = line_iterator[i];
            if boundary_line.contains("Content-Type"){
                break;
            } else {
                i += 1;
            }
        }
        // let boundary_line = get_boundary_line(line_iterator_string);
        // println!("{:#?}", boundary_line);
        let boundary_vec: Vec<&str> = boundary_line.split(" ").collect();
        let boundary = boundary_vec[2];
        let mut boundary = String::from(boundary); //&str to String
        let _boundary2 = boundary.pop();
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);
        let _boundary2 = boundary.remove(0);

        // let boundary2 = boundary.pop();
        // println!("{:#?}", boundary);
        
        i = 0;
        let mut filename_line;
        loop{
            filename_line = line_iterator[i];
            if filename_line.contains("filename"){
                break;
            } else {
                i += 1;
            }
        }
        let filename_vec: Vec<&str> = filename_line.split(" ").collect();
        let filename = filename_vec[3];
        // let filename2 = &filename.to_string();
        let mut filename = String::from(filename); //&str to String
        let _filename2 = filename.pop();
        let _filename2 = filename.pop();
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        let _filename2 = filename.remove(0);
        // println!("{:#?}", filename);

        

        let offset = find_payload_index(&buffer);
        i = 0;
        loop{
            let _buffer2 = buffer.remove(0);
            if i == offset{break;}
            i += 1;
        }

        let _buffer2 = buffer.remove(0);
        let _buffer2 = buffer.remove(0);
        let _buffer2 = buffer.remove(0);
        let _buffer2 = buffer.remove(0);

        let offset = find_payload_index(&buffer);
        i = 0;
        loop{
            let _buffer2 = buffer.remove(0);
            if i == offset{break;}
            i += 1;
        }

        let _buffer2 = buffer.remove(0);
        let _buffer2 = buffer.remove(0);
        let _buffer2 = buffer.remove(0);

        // let buffer2 = buffer.remove(0);
        // println!("{:#?}", String::from_utf8_lossy(&buffer[..]));
        
        let mut file = std::fs::File::create(filename).expect("create failed");
        let _write_result = file.write_all(&buffer).expect("write failed");
        let flush_result = file.flush();
        // println!("{:#?}", flush_result.unwrap());
        // let result = copy(&mut buffer, &mut writer);
  
    }
}


fn find_payload_index(buffer: &[u8]) -> usize{
    let _i = 0;
    for i in 0..buffer.len() {
        if buffer[i..].starts_with(b"\r\n\r\n") {
            // println!("{:#?}", i);
            return i;
        }
    }
    panic!("malformed buffer without the sequence");
}
