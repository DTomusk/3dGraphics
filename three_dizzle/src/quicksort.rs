pub fn quicksort(mut data: &mut [f64]) {
    if data.len() <= 1 {
        return
    };
    let mut pivot: usize = data.len()/2;
    // want to iterate counters either side of the pivot
    let mut first_counter = 0;
    let mut second_counter = pivot + 1;

    println!("Start sort, first: {}, second: {}", first_counter, second_counter);
    while (first_counter < pivot) | (second_counter < data.len()) {
        // if the current things need to be swapped then swap them
        if (first_counter < pivot) & (second_counter < data.len()) {
            println!("Both counters inside bounds: {} & {}", first_counter, second_counter);
            if (data[first_counter] > data[pivot]) & (data[second_counter] < data[pivot]) {
                println!("Two elements swapped: {} & {}", &data[first_counter], &data[second_counter]);
                swap(&mut data, first_counter, second_counter);
                first_counter+=1;
                second_counter+=1;
            } else if data[first_counter] > data[pivot] {
                println!("Nothing for first to swap with, increasing second");
                second_counter+=1;
            } else if data[second_counter] < data[pivot] {
                println!("Nothing for second to swap with, increasing first");
                first_counter+=1;
            } else {
                println!("No swaps for either, increasing both counters");
                first_counter += 1;
                second_counter += 1;
            };

        } else if first_counter < pivot {
            if data[first_counter] > data[pivot] {
                swap(&mut data, pivot - 1, pivot);
                swap(&mut data, pivot, first_counter);
                pivot -= 1;
            } else {
                first_counter += 1;
            };
        } else {
            if (data[second_counter] < data[pivot]) & (first_counter == data.len()) {
                swap(&mut data, pivot + 1, pivot);
                swap(&mut data, pivot, first_counter);
                pivot += 1;
            // if one thing needs to be moved hold on to it and wait for the other
            } else {
                second_counter += 1;
            };
        };





/*
        else  else   else if (data[first_counter] > data[pivot]) & (first_counter == pivot - 1) {
            swap(&mut data, first_counter, pivot);
            first_counter += 1;
            pivot -= 1;
        } else if (data[second_counter] < data[pivot]) & (second_counter == pivot + 1) {
            swap(&mut data, second_counter, pivot);
            second_counter += 1;
            pivot += 1;

        */
        // if one thing needs to be moved and the other has run out do the weird swap thing

        // that should leave just the case where neither need to be moved and are still in bounds
    };
    quicksort(&mut data[..pivot]);
    quicksort(&mut data[pivot..]);
}

pub fn swap(data: &mut[f64], first: usize, second: usize) {
    let temp = data[first];
    data[first] = data[second];
    data[second] = temp;
}
