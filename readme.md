<h1>SORT ALGORITHMS</h1>
<strong><i>Obs:. This just a briefing of the algorithm!!!</i></strong>
<div>
    <h3>SELECTION SORT</h3>
    Selection Sort is an algorithm that use order by selection.<br/>
    Worst, Medium and Best Case Complexity: <strong>O(n²)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Selection_sort">https://en.wikipedia.org/wiki/Selection_sort</a><br/>
    
    
    
    extern crate sort_algorithms;

    use sort_algorithms::selection_sort;

    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        selection_sort(&mut arr, |a, b| a < b);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
    

</div>

<div>
    <h3>BUBBLE SORT</h3>
    Bubble sort is an algorithm that use order by comapare values.<br/>
    Worst, Medium Case Complexity: <strong>O(n²)</strong><br/>
    Best Case Complexity: <strong>O(n)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Bubble_sort">https://en.wikipedia.org/wiki/Bubble_sort</a><br/>

    
    extern crate sort_algorithms;

    use sort_algorithms::bubble_sort;

    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        bubble_sort(&mut arr, |a, b| a < b);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
    

</div>

<div>
    <h3>QUICK SORT</h3>
    Quick sort is an algorithm that use strategy divide to conquer.<br/>
    Worst Case Complexity: <strong>O(<i>n</i>²)</strong><br/>
    Medium Case Complexity: <strong>O(<i>n</i>)</strong><br/>
    Best Case Complexity: <strong>O(<i>n</i> log <i>n</i>)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Quicksort">https://en.wikipedia.org/wiki/Quicksort</a><br/>
    
    
    extern crate sort_algorithms;

    use sort_algorithms::quick_sort;

    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        quick_sort(&mut arr, |a, b| a < b);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
    

</div>

<div>
    <h3>HEAP SORT</h3>
    Heap sort is an generalist algorithm that use the strategy order by selecion.<br/>
    Worst Case Complexity: <strong>O(<i>n</i> log <i>n</i>)</strong><br/>
    Medium Case Complexity: <strong>O(<i>n</i> log <i>n</i>)</strong><br/>
    Best Case Complexity: <strong>O(<i>n</i> log <i>n</i>)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Heapsort">https://en.wikipedia.org/wiki/Heapsort</a><br/> 
    
    
    extern crate sort_algorithms;

    use sort_algorithms::heapsort;

    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        heapsort(&mut arr);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
    

</div>

<div>
    <h3>MERGE SORT</h3>
    Merge sort is an algorithm that use the strategy order by comparation and divide to conquer.<br/>
    Worst Case Complexity: <strong>O(<i>n</i> log <i>n</i>)</strong><br/>
    Medium Case Complexity: <strong>O(<i>n</i> log <i>n</i>)</strong><br/>
    Best Case Complexity: <strong>O(<i>n</i>)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Merge_sort">https://en.wikipedia.org/wiki/Merge_sort</a><br/>

    
    extern crate sort_algorithms;

    use sort_algorithms::merge_sort;

    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        merge_sort(&mut arr, |a, b| a < b);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
     

</div>

<div>
    <h3>RADIX SORT</h3>
    Radix sort is an algorithm that use the strategy non-comparative.<br/>
    Worst Case Complexity: <strong>O(<i>n</i>)</strong><br/>
    Medium Case Complexity: <strong>O(<i>n</i>)</strong><br/>
    Best Case Complexity: <strong>O(<i>n</i>)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Radix_sort">https://en.wikipedia.org/wiki/Radix_sort</a><br/>
    <p><i><strong>Can only be used to sort lists of positive integers as key</strong></i></p><br/>

    
    extern crate sort_algorithms;

    use sort_algorithms::radix_sort;

    fn main() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        radix_sort(&mut arr, |&a| a);
        println!("{:?}", &arr);
        assert_eq!(arr, [ 0, 1, 2, 3, 4, 5, 6, 7]);
    }
    
</div>


<div>
    <h3>INSERTION SORT</h3>
    Insertion sort is an algorithm that use strategy where catch one element and compare against orthers.<br/>
    Worst, Medium Case Complexity: <strong>O(n²)</strong><br/>
    Best Case Complexity: <strong>O(n)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Insertion_sort">https://en.wikipedia.org/wiki/Insertion_sort</a><br/>

    
    extern crate sort_algorithms;

    use sort_algorithms::insertion_sort;

    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        insertion_sort(&mut arr, |a, b| a < b);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
    

</div>

<div>
    <h3>COCKTAIL SHAKER SORT</h3><br/>
    Cocktail Shaker Sort is an algorithm is a derivation from bubble sort.<br/>
    Worst, Medium Case Complexity: <strong>O(n²)</strong><br/>
    Best Case Complexity: <strong>O(n)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Cocktail_shaker_sort">https://en.wikipedia.org/wiki/Cocktail_shaker_sort</a><br/>

    
    extern crate sort_algorithms;

    use sort_algorithms::cocktail_shaker_sort;


    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        cocktail_shaker_sort(&mut arr, |a, b| a < b);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
    
    
</div>

<div>
    <h3>GRAVITY SORT / BEAD SORT</h3><br/>
    Gravity Sort is an algorithm that use the strategy of <i>Natural Sorting</i>.<br/>
    Worst, Medium and Best Case Complexity: <strong>O(n)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Bead_sort">https://en.wikipedia.org/wiki/Bead_sort</a><br/>
    <p><i><strong>Can only be used to sort lists of positive integers as key</strong></i></p><br/>

    
    extern crate sort_algorithms;

    use sort_algorithms::gravity_sort;


    fn main() {
        let mut arr = vec![9, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        gravity_sort(&mut arr, |&a| a);
        println!("{:?}", &arr);
        assert_eq!(arr, [ 0, 1, 2, 3, 4, 5, 6, 9]);
    }
    

</div>

<div>
    <h3>COUNTING SORT</h3>
    Counting Sort is an algorithm that use the strategy of  it uses key values as indexes into an array and 
    the <strong>Ω(<i>n</i> log <i>n</i>)</strong> lower bound for comparison sorting will not apply.<br/>
    Worst, Medium and Best Case Complexity: <strong>O(n)</strong><br/>
    <a href="https://en.wikipedia.org/wiki/Counting_sort">https://en.wikipedia.org/wiki/Counting_sort</a><br/>
    <p><i><strong>Can only be used to sort lists of positive integers as key</strong></i></p><br/>

    
    extern crate sort_algorithms;

    use sort_algorithms::counting_sort;

    fn main() {
        let mut arr = vec![7, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        counting_sort(&mut arr, |&a| a);
        println!("{:?}", &arr);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    
</div>


<div>
    <h3>FLASH SORT</h3>
    Flash Sort is an algorithm that use the strategy that you can compute the approximate final position directly from the element value, 
    with no comparisons.<br/>
    Worst, Medium and Best Case Complexity: <strong>O(n)</strong>
    <a href="http://www.neubert.net/Flapaper/9802n.htm">http://www.neubert.net/Flapaper/9802n.htm</a>

    
    extern crate sort_algorithms;

    use sort_algorithms::flash_sort;
    fn main() {
        let mut arr = vec![-1, 6, 5, 2, 4, 3, 1, 0];
        println!("{:?}", &arr);
        flash_sort(&mut arr, |&a| a);
        println!("{:?}", &arr);
        assert_eq!(arr, [-1, 0, 1, 2, 3, 4, 5, 6]);
    }
   
</div>




