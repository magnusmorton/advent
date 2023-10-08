
use ndarray::Array;

fn main() {
    let str = std::fs::read_to_string("input.txt").unwrap();
    let lines = str.lines();
    let count = lines.clone().fold(0, |acc, _| acc + 1);
    let mut arr = Array::<u32,_>::zeros((count, count));
    let mut visible = Array::<u64,_>::zeros((count, count));

    for (i,line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            arr[[i,j]] = c.to_digit(10).unwrap();
        }    
    }

    for i in 0..count {
        visible[[0,i]] = 1;
        visible[[count-1, i]] = 1;
        visible[[i, 0]] = 1;
        visible[[i, count-1]] = 1;
    }
    
    for i in 0..count {
        let (mut lmax, mut rmax, mut umax, mut dmax) = (0,0,0,0);

        for j in 0..count {
            let (currl, currr, curru, currd) = (arr[[i, j]], arr[[i, count-1-j]], arr[[j, i]], arr[[count-1-j, i]]);
            if currl > lmax {
                lmax = currl;
                visible[[i,j]] = 1;
            }
            if currr > rmax {
                rmax = currr;
                visible[[i, count-1-j]] = 1;
            }
            if curru > umax {
                umax = curru;
                visible[[j, i]] = 1;
            }
            if currd > dmax {
                dmax = currd;
                visible[[count-1-j, i]] = 1;
            }
        }
    }
    println!("{}", visible.sum());

    let mut max = 0;
    for ((x,y), el) in arr.indexed_iter() {

        let (mut seenu, mut seend, mut seenl, mut seenr): (u64, u64, u64, u64) = (0,0,0,0);
        let mut yu = y;
        while yu != 0 {
            seenu += 1;
            yu-=1;
            if arr[[x, yu]] >= *el {
                break;
            }
        }

        let mut yd = y;
        while yd != count-1 {
            seend += 1;
            yd+=1;
            if arr[[x, yd]] >= *el {
                break;
            }
        }

        let mut xl = x;
        while xl != 0 { 
            seenl += 1;
            xl-=1;
            if arr[[xl, y]] >= *el {
                break;
            }
        }

        let mut xr = x;
        while xr != count-1 {
            seenr += 1;
            xr+=1;
            if arr[[xr, y]] >= *el {
                break;
            }
        }

        let prod = seenu * seend * seenl * seenr;
        if prod > max {
            max = prod
        }
    }
    println!("max: {}", max)

}
