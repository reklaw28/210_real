<h1><b> overview</b></h1>
<p> This program is designed to rank debaters in a similar way to how page rank worked for the early Google search engine. It constructs a weighted graph and solves for the steady state and ranks the debaters by how high of a probability there is for their node to be visited. This is more heavily weighted on the losses side because it increases the odds of visiting a node if a lot of people have lost to them and if a lot of people have lost to the nodes connecting to them. Then it returns the top 15 debaters for a data set of the 2017-2018 season in college policy </p>
<h1><b>Example output given the data set linked (this reached the steady state fully)</b></h1>
<p>Vertice = "Emory: Giampetruzzi & Kessler", and prob/rank = 0.0161<br>
Vertice = "Michigan: Pierry & Rabbini", and prob/rank = 0.0112<br>
Vertice = "Dartmouth: Tambe & Vergho", and prob/rank = 0.0099<br>
Vertice = "Southern California: Kuffour & Sun", and prob/rank = 0.0091<br>
Vertice = "Kentucky: Di & Griffith", and prob/rank = 0.0087<br>
Vertice = "Wake Forest: Bittner & Davidson", and prob/rank = 0.0083<br>
Vertice = "Kansas: McMahon & Scott", and prob/rank = 0.0082<br>
Vertice = "Texas: Desai & Kashyap", and prob/rank = 0.0079<br>
Vertice = "Michigan: Margolin & Muse", and prob/rank = 0.0071<br>
Vertice = "Northwestern: Deo & Fridman", and prob/rank = 0.0071<br>
Vertice = "Dartmouth: Shankar & Vergho", and prob/rank = 0.0071<br>
Vertice = "Michigan: Phil & Pierry", and prob/rank = 0.0069<br>
Vertice = "UC Berkeley: Eusterman & Eusterman", and prob/rank = 0.0064<br>
Vertice = "Pittsburgh: Mendoza & Osei", and prob/rank = 0.0061<br>
Vertice = "Emory: Pak & Rajagopal", and prob/rank = 0.0058<br>
</p>
<h1><b>How to use</b></h1>
<p> Simply put the file path for the data set that you have into the read file function. Then ensure that the csv contains accurate data that has the winner of the round in the left-most column and the loser in the right-most column and that they are both strings. Once that is confirmed adjust how many random walks you'd like to perform (by changing the walks and steps_per_walk variables to a different integer I picked my numbers based on the number of nodes and scaled up from there) as the current numbers have a very long run time because it is navigating 121,200,000 times.  As long as those are correct the only other thing you should need to do is call cargo run --release. </p>
<h1><b>Nitty Gritty</b></h1>
<p> First we open the data using the read_file function that returns a vector of tuples containing strings in the first two positions. Then we create the point struct where we assign each win which is at the 0 index in the tuple and loss at the 1 index. We then create a vector of these points called all_points where we push each point into it. Then we call ad_list function which creates a adjacency list using the points(see mods for more info). Then we create a hashmap where we can find the index of a given key in the adjacency list because it's location is random because of the graph construction using hashmaps. Then we define how many random walks and steps we want to do. Then we create a vector of 0's called probabilities that's the size of the amount of nodes so that we can later mark each node we visited using the graph. Then we begin to iterate through walks and define a random point at the start of it each time it loops using thread rng with a range of every possible index using the size variable. Then we iterate through steps which first grab the first vector using the adjacency list and the randomly selected index and sets that = to temp. Then we find the min and max for temp using the min,max function in the open module. We then make a new temporary variable named temp2 which stores the scaled temp so that we don't affect the original values in the adjacency list. We scale them using the scale function in the open module. Then we create a jump variable generating a random number between the min and max of the current vertex. We then initialize our two store vectors. if temp2 is empty which it shouldn't be then we just randomly select a new node to jump to. Then we check if jump is positive or negative and then we do our random selection in either the strictly positive entries or the values that are 0 or negative. We put those into a vector that contains either the strictly positive or not values. Then we increase the probability that a value with a higher weight gets selected by adding copies of it to the vector = to its weight. Once that is completed we then generate a random number in the range of 0 to the length of the store2 vector. That new number becomes = to the nex_vertex. Then we set the current vertex to the index of the next vertex using the indexs hashmap to find it. Then we add 1 to the same index in the probability matrix to show that we visited.We then iterate till we've completed every walk. We then create a new binary heap which will be used for sorting later. Then we create a probability vector and iterate through the probability matrix. We make sure to keep track of the index but in the probability vector, we put the probability in the first index of the tuple because binary heaps order by the first number in the tuple.We then iterate through the probability vector and push every value to the binary heap. We then pop the first 15 values and divide them by the size of the matrix and print them in order to return the probability of selecting them and what they are.    </p>
<h1><b>Mods</b></h1>
<h3>read_file</h3>
<p>uses the standard crate File to open and then read the file in the file path you pass to the function. Returns a vector of tuples that contain two strings each</p>
<h3>min</h3>
<p>uses a closure and cmp to find the minimum value in a vector of tuples that are passed from main and returns an int that is the minimum value</p>
<h3>Max</h3>
<p>does the same as min above but for max</p>
<h3>scale</h3>
<p>takes a vector of tuples that contain a string and i32 and an i32 which is the minimum value in the vector of tuples. Then it adds that minimum value to each negative value in the tuples to make them more likely to be selected</p>
<h3>ad_list</h3>
<p>used to construct our adjacency list. Takes a vector of points as it input. We then iterate through the vector and assign each WIN and LOSS to either the win hashmap or loss hashmap. If the value is already in the hashmap then we just append the corresponding win or loss to what's contained in the hashmap to be accessed by the key which is the repeated value.Then we make a vector that contains every key except for the headers at the top of the file and when adding loss keys in we make sure that the loss key is not already in the vector before pushing it using a closure. Then we iterate through each key in the vector and add or subtract 1 for each time a team appears in its corresponding hashmap for the given key. And if it's not already in the adjacency list it will append it to the vector that is in the adjacency list that corresponds with the key. Finally, we return the adjacency list which is a vector that contains tuples with the first index of the tuple being the key and the second being the vector that contains tuples that have the teams that correspond to the key in the hashmap and has the weights on them. </p>
<h1><b>Tests</b></h1>
<h3>graph right</h3>
<p>This constructs a graph and makes sure that all of the weighted values added together = 0 because this graph should be always and summed up to 0 so if it doesn't somethings gone wrong</p>
<h3>scale right</h3>
<p>This ensures that the scaling function is working properly by using a fake data set and comparing the scaled value to what it should be</p>
