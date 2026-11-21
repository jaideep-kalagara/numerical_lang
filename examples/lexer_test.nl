let x: float = 1.57
let y: float = 1.57
let z: float = x + y
; This function 
func xyz(x: float, y: float): float {
    return x + y
}

; Entry point of the program, returns the exit code of the program
func main(): integer {
    if (2.0 * y == z) {
        let a: float = xyz(x, y)
    }

    let str: string = "This is a test string literal"

    return 0
}