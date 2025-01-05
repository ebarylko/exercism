let prepend str c = String.make 1 c ^ str

let reverse_string  =
    String.fold_left prepend ""
