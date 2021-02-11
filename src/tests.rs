use crate::*;

#[test]
fn test_simple_maze() {
    let maze = vec![
        1, 0, 1, 1, 1,
        1, 0, 0, 1, 1,
        1, 1, 0, 0, 1,
        1, 0, 0, 1, 1,
        1, 1, 1, 1, 1
    ];
    let maze = Maze {
        data: maze.iter().map(|&i| i != 0).collect(),
        n: 5,
        m: 5,
    };
    let mut path = maze.solve((3, 1)).unwrap();
    // Pop, go backwards.
    assert_eq!(path.pop(), Some((0, 1)));
    assert_eq!(path.pop(), Some((1, 1)));
    assert_eq!(path.pop(), Some((1, 2)));
    assert_eq!(path.pop(), Some((2, 2)));
    assert_eq!(path.pop(), Some((3, 2)));
    assert_eq!(path.pop(), Some((3, 1)));
    assert_eq!(path.pop(), None);
}

#[test]
fn test_blocked_maze() {
    let maze = vec![
        1, 1, 1, 1, 1, 1,
        1, 0, 0, 1, 0, 1,
        1, 0, 1, 1, 0, 1,
        1, 0, 0, 1, 0, 1,
        1, 1, 1, 1, 0, 1,
        1, 0, 0, 1, 0, 1,
        1, 1, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1,
    ];
    let maze = Maze {
        data: maze.iter().map(|&i| i != 0).collect(),
        n: 8,
        m: 6,
    };
    let path = maze.solve((1, 2));
    assert!(path.is_none());
}

#[test]
fn test_maze_generation() {
    for _ in 0..4 {
        let maze = Maze::generate(256, 256);
        let mut path = None;
        for i in 0..maze.n {
            for j in 0..maze.m {
                if !maze[(i, j)] {
                    path = maze.solve((i, j))
                }
            }
        }
        assert!(path.is_some());
    }
}
