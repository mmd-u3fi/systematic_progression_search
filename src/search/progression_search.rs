use super::fringe::Fringe;

struct ProgressionSearch<U> 
where U: Fringe {
    fringe: U
}