use std::cmp::min;
use std::fs;
use std::path::{Path, PathBuf};
use catch_input::input;
fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

fn t(max:usize, src: &str) {
    let paths = fs::read_dir(&src).unwrap();
    let mut new_paths = vec![];
    for path in paths{
        let i = path.unwrap().path();
        let relative = i.strip_prefix(src).unwrap();
        let mut newname = "".to_owned();
        if relative.to_string_lossy().len()<max{
            //add max - min 0's
            for _i in 0..max-relative.to_string_lossy().len(){
                let temp=("0".to_owned()+ &*newname);
                newname = (*temp).parse().unwrap();
            }

        }
        newname = newname + &*relative.to_string_lossy();
        //println!("{}",&newname);
        //println!("{}",&newname.as_str());
        let new_path = change_file_name(i, newname.as_str());
        println!("New path: {}", &new_path.display());
        assert_eq!(new_path, Path::new(src).join(newname));
        new_paths.push(new_path);
    }

    let paths = fs::read_dir(&src).unwrap();
    for paths in paths.zip(new_paths.iter()){
        let (old, new) = paths;

        // println!("Old: {}, New: {}", &old.unwrap().path().display(), &new.display());
        fs::rename(&old.unwrap().path(), &new).unwrap();

    }


}

fn main() {
    println!("\nPlease note that this might take a while. Please wait until the command finishes");
    let p = input!("Enter directory path: ");


    let paths = fs::read_dir(&p).unwrap();
    let src = &p;
    let mut max_len = 0;
    let mut min_len = 500;
    for path in paths {
        //println!("Name: {}", &path.unwrap().path().display());
        let i = path.unwrap().path();
        let relative = i.strip_prefix(src).unwrap();
        println!("Relative: {}, len: {}", &relative.display(), &relative.to_string_lossy().len());
        if relative.to_string_lossy().len() > max_len {
            max_len = relative.to_string_lossy().len();
        }else if relative.to_string_lossy().len() < min_len {
            min_len = relative.to_string_lossy().len();
        }
    }
    // println!("Max len: {}", max_len);
    // println!("Min len: {}", min_len);
    t(max_len, src);

let new_paths = fs::read_dir(&p).unwrap();
    for path in new_paths {
        println!("Name: {}", &path.unwrap().path().display());
    }
}
