# disconnected graph: path 0-1-2 plus isolated node A
# Only edge-endpoint nodes exist: 0, 1, 2 (A has no edges → not in graph!)
# Wait: nx.read_edgelist only adds nodes from endpoints
# To get a disconnected graph with isolated node in read_edgelist, we need
# to use a comment node trick — but actually nx.read_edgelist ignores isolates.
# For disconnect: two separate components via edges
# Component1: 0-1-2, Component2: 10-11
# networkx: diameter → NetworkXError (not connected)
0 1
1 2
10 11
