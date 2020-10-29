// Maps (i,j) into the corresponding index in a flat vector.
//
// Directed edges:
//      At each vertex insertion, a new row, column and a slot for diagonal position must be allocated.
//      If we store matrix row by row, we will face problem when adding a new column.
//      If we store matrix column by column, we will face problem when adding a new row.
//      Since we need to both add a row and a column, both mappings cause trouble.
//      For example if we store matrix row by row:
//      _______
//      |1|2|3|
//      |4|5|6|     -->     [1, 2, 3, 4, 5 ,6, 7, 8, 9]
//      |7|8|9|
//      -------
//
//      Adding vertex with id=3:
//      _____________
//      | 1| 2| 3|10|
//      | 4| 5| 6|11|     -->     [1, 2, 3, {10}, 4, 5 ,6, {11}, 7, 8, 9, {12}, {13}, {14}, {15} , {16}]
//      | 7| 8| 9|12|
//      |13|14|15|16|
//      -------------
//      * Newly inserted items are marked with {}
//
//      When we want to add vertex with id=3, we need to add row=3, column=3 and the diagonal slot [3,3].
//      Therefore we need to allocate between 3 and 4, 6 and 7  and add the rest at the end of the vector.
//      Inserting element between two elements cause the vector to shift other elements which is not good for performance.
//      Therefore we want a mapping that enables us to only append at the end of the vector upon each vertex insertion.
//
//      In order to allocate space at the end of the vector at each vertex insertion,
//      mapping below is proposed:
//      _______
//      |1|2|3|
//      |4|5|6|     -->     [1, 4, 5, 2, 7, 8, 9, 6, 3]
//      |7|8|9|
//      -------
//
//      Adding vertex with id=3:
//      _____________
//      | 1| 2| 3|10|
//      | 4| 5| 6|11|     -->     [1, 4, 5, 2, 7, 8, 9, 6, 3, {13} , {14}, {15}, {16}, {12}, {11}, {10}]
//      | 7| 8| 9|12|
//      |13|14|15|16|
//      -------------
//      * Newly inserted items are marked with {}
//
//      With this mapping when adding the vertex with id=3, the newly allocated row, column and diagonal slot will be added at the end of the vector.
//
// Undirected edges:
//      A simple row by row storage will do.
pub fn from_ij(mut i: usize, mut j: usize, is_directed: bool) -> usize {
    if is_directed {
        let k = std::cmp::max(i, j);

        (k - i) + j + k * k
    } else {
        if j > i {
            std::mem::swap(&mut i, &mut j);
        }
        // prevent division: i * (i + 1) is definitely an even number thus i * (i + 1) / 2 == i * (i + 1) >> 1
        (i * (i + 1) >> 1) + j
    }
}
