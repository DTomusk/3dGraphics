pub fn quicksort(mut data: &mut [f64]) {
    let mut pivot: usize = data.len()/2;
    // want to iterate counters either side of the pivot
    let mut first_counter = 0;
    let mut second_counter = pivot + 1;
    while (first_counter < pivot) | (second_counter < data.len()) {
        // if the current things need to be swapped then swap them
        if (first_counter < pivot) & (second_counter < data.len()) {
            if (data[first_counter] > data[pivot]) & (data[second_counter] < data[pivot]) {
                swap(&mut data, first_counter, second_counter);
                first_counter+=1;
                second_counter+=1;
            } else if data[first_counter] > data[pivot] {
                second_counter+=1;
            } else if data[second_counter] < data[pivot] {
                first_counter+=1;
            } else {
                first_counter += 1;
                second_counter += 1;
            };
        } else if first_counter < pivot {
            if (data[first_counter] > data[pivot]) & (first_counter == pivot - 1) {
                swap(&mut data, pivot, first_counter);
                pivot -= 1;
            } else if data[first_counter] > data[pivot] {
                swap(&mut data, pivot - 1, pivot);
                swap(&mut data, pivot, first_counter);
                pivot -= 1;
            } else {
                first_counter += 1;
            };
        } else {
            if (data[second_counter] < data[pivot]) & (second_counter == pivot + 1) {
                swap(&mut data, pivot, second_counter);
                pivot += 1;
            } else if data[second_counter] < data[pivot] {
                swap(&mut data, pivot + 1, pivot);
                swap(&mut data, pivot, second_counter);
                pivot += 1;
            // if one thing needs to be moved hold on to it and wait for the other
            } else {
                second_counter += 1;
            };
        };
    };
    if data.len() >= 2 {
        quicksort(&mut data[..pivot]);
        quicksort(&mut data[pivot..]);
    };
}

pub fn swap(data: &mut[f64], first: usize, second: usize) {
    let temp = data[first];
    data[first] = data[second];
    data[second] = temp;
}
