use std::fs::File;
use std::io;
use std::io::Read;
use crate::WindowResult::{Ascending, Descending, Wrong};

fn main() -> io::Result<()> {
    let mut file = File::open("input/day2/reports.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut list1 = Vec::new();

    for line in contents.lines() {
        let www: Vec<&str> = line.split_whitespace().collect();
        let words: Vec<i32> = www.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        list1.push(words);
    }

    let mut counter = 0;

    for i in 0..list1.len() {
        if is_report_safe(&list1[i]) {
            counter = counter + 1;
        }
    }

    println!("{:?}", counter);
    Ok(())
}

#[derive(PartialEq)]
enum WindowResult {
    Wrong,
    Ascending,
    Descending
}

fn analyze_report(report: &Vec<i32>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    fn check_window(w: &[i32]) -> WindowResult {
        if w[0] < w[1] && w[1] - w[0] < 4 {
            return Ascending
        } else if w[0] > w[1] && w[0] - w[1] < 4 {
            return Descending
        } else {
            return Wrong
        };
    }

    let windows_results = report.windows(2).map(check_window);

    let mut wrong_windows_indices = Vec::new();
    let mut asc_windows_indices = Vec::new();
    let mut desc_windows_indices = Vec::new();

    for (i, result) in windows_results.enumerate() {
        match result {
            Wrong => wrong_windows_indices.push(i),
            Ascending => asc_windows_indices.push(i),
            Descending => desc_windows_indices.push(i)
        }
    }

    return (wrong_windows_indices, asc_windows_indices, desc_windows_indices)
}

fn is_report_safe_intolerant(report: &Vec<i32>) -> bool {
    let (wrong_windows_indices, asc_windows_indices, desc_windows_indices) = analyze_report(report);

    return wrong_windows_indices.len() == 0 && (asc_windows_indices.len() == 0 || desc_windows_indices.len() == 0)
}

fn is_report_safe_with_element_removed(report: &Vec<i32>, elem_to_remove_idx: usize) -> bool {
    let mut report_with_elem_removed = report.clone();
    report_with_elem_removed.remove(elem_to_remove_idx);

    return is_report_safe_intolerant(&report_with_elem_removed)
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let (w, a, d) = analyze_report(report);
    let mut wrong_windows_indices = w;
    let mut asc_windows_indices = a;
    let mut desc_windows_indices = d;

    let asc_count = asc_windows_indices.len();
    let desc_count = desc_windows_indices.len();

    //Replace asc or desc (minority) with Wrong
    if asc_count < desc_count {
        wrong_windows_indices.append(&mut asc_windows_indices);
        wrong_windows_indices.sort();
    } else {
        wrong_windows_indices.append(&mut desc_windows_indices);
        wrong_windows_indices.sort();
    }

    let wrong_count = wrong_windows_indices.len();

    return if wrong_count > 2 {
        false
    } else if wrong_count == 2 {
        if wrong_windows_indices[1] - wrong_windows_indices[0] == 1 {
            // Indeksy obok siebie
            let elem_to_remove_idx = wrong_windows_indices[1];
            is_report_safe_with_element_removed(report, elem_to_remove_idx)
        } else {
            false
        }
    } else if wrong_count == 1 {
        let elem_to_remove_idx1 = wrong_windows_indices[0];
        let elem_to_remove_idx2 = elem_to_remove_idx1 + 1;
        is_report_safe_with_element_removed(report, elem_to_remove_idx1) || is_report_safe_with_element_removed(report, elem_to_remove_idx2)
    } else {
        true
    }
}
