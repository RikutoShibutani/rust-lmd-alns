The attached instances are instances of the vehicle routing problem with time windows, multiple deliverymen and two-level routing (VRPTWMD2R) proposed by Senna et. al (2024).

In this problem, there are clusters of customers.
The instances are based on the Solomon (1987) instances for the VRPTW.
The distances and travel times are calculated as euclidean distances truncated to integers.
The instances are structured as follows.

Instace file:

----------------------------------------------------------------------------------------
INSTANCE NAME

# CLUSTERS	# CUSTOMERS

VEHICLE
NUMBER     CAPACITY
  #           #

// Clusters information (parking locations). The first line represents the depot.

CLU NO.  XCOORD.   YCOORD.    DEMAND    READY TIME  DUE DATE   SERVICE  TIME
 
// Customers information.
 
CUST NO.  XCOORD.   YCOORD.    DEMAND   READY TIME  DUE DATE   SERVICE TIME  CLUSTER

----------------------------------------------------------------------------------------
