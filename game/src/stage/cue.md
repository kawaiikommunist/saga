Idea:

Yggdrasil {
    Contains all of the data to fully calculate the turn passing
    Contains a graph that connects all of the input data for the turn passing to the output data
    Each edge is a directed edge with a value of TypeId
    Each node is a function that accepts arguments matching the TypeIds of the incoming edges and spits out a return type of the outgoing edges
    Some nodes are external input (they need to be "filled" with data from outside the system)
    some nodes are external output (they "return" data from within the graph)
    Some nodes are internal (they only depend on values produced inside of the system)
    The graph is acyclic and can be iterated through in parallel to calculte all of the outgoing nodes
}
