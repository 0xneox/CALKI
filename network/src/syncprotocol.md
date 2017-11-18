# Synchronization protocol

Keyword explanation:
* Implementer: refers to the realization of the structure or logic of the synchronization protocol.
* CALKI synchronization principle: <br>
Give a chestnut: <br>
A, B, C, D four nodes, if the global height is 150, A node offline at 100, Now A node restart, you need to sync. <br>
The blocks to be synchronized are: 101, 102, 103, 104, ... 149, 150 height blocks. <br>
Because of the various reasons why blocks of consensus are generated, the key points to keep in mind when synchronizing blocks are: 102 for synchronization when 101 is needed, 102 for synchronization, 103, and so on.
When synchronizing the last block, 150, requires a virtual block height of std :: usize :: MAX, these are all implemented in the synchronization protocol. See the .Forward :: reply_syn_req method for details. <br>
Comparison of image description: "<-" symbol is "promote, verify" means
Ie 102 <- 102 <-103 <- 104 .... 149 <- 150 <- std :: usize :: MAX <br>

## A brief description
Agreement is the main chain state synchronization, other synchronization temporarily do not consider the contents of what:

Broadcast status
2. Synchronize the request
3. Processing the request
4 synchronization status strategy
Subcontracting strategy
6. Sort strategy
7. Asynchronous synchronization status
8. lost packet \ bad packet processing
9. Time-out processing

## main description to achieve
Before describing the implementation: Please bring the following prerequisites.
1. Other nodes to the state and the block is not credible, but the implementer can not do the synchronization work, how to avoid this contradiction?
2. Under what conditions to end the synchronization?
3. If you are in synchronization chain or network disconnected how to do?

#### broadcast status
Receive the state of its own node, greater than or equal to the previous state to broadcast, mainly to prevent attacks.

#### Synchronization request
Triggering timing for initiating synchronization request.
1. When the status from other nodes is greater than the status of the current node, the implementor initiates a synchronization request.
2. When the status from own node is smaller than the status of other nodes, the implementor initiates a synchronization request.

Trigger the end of the synchronization
1 is the current node status consistent with the global state, the end of synchronization
2. Conclude the synchronization when all the heights (buffers) are reached.

#### Process the request
The implementor receives the synchronized blocks, performs the split package, sorts, and then acquires the new set K's package to send to the current node's chain.

#### Synchronization status policy
Synchronization chain state strategy: the group K good package sent to the chain, waiting for the state of the chain (asynchronous implementation), if there is a good package K, and then sent to the chain, in order to cycle, to update the status of the chain.

#### Subcontracting / Sorting Strategy
Initiate a synchronization request to other nodes, initiated by an iteration step, that is, step = 20, and each packet's request is initiated randomly to any of the other nodes.
Due to the transmission of the network, the synchronizer gets the multiple responses of the corresponding request, the order is not the same, therefore, we need to sort the received packet.
In the synchronization are well preserved and arranged in order height block, according to the step number, followed by synchronization to the chain module.

Since chain execution needs to execute a transaction when it is added, the synchronization to other nodes is faster than the synchronization to the chain block.

#### Synchronization status is asynchronous
The logic that initiates the synchronization request and the synchronization height is asynchronous.
You can initiate a synchronization request while the height is being synchronized while the block is synchronized with the reception process.

#### packet loss \ bad packet processing
If the chain internal verification fails, the entire group packet is dropped and the current state of the broadcast is broadcast. The implementer needs to remember (the current node) chain about the height to be synchronized with the height of the broadcast:
If equal, continue to get the next packet to the chain.
If less than error, restart the synchronization operation.
If greater than, chain packet loss, the phenomenon of bad package, according to the current state of the chain, initiated a step, and cover the current Sync with the same height value.

#### Timeout processing
What to do when you initiate a synchronization request for a timeout? How to interrupt? How to re-start?
Need to rely on the latest next to a global state to break. That time-out mechanism
