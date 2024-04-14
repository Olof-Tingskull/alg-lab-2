# Lab 2 - Algorithms and Complexity (DD2352) 

## Theory

### 1 Write, in some notation of your choice, a solution to the yes-instance of the casting problem in the example above
- Role 1 is played by actor 1
- Role 2 is played by actor 3
- Role 3 is played by actor 1
- Role 4 is played by actor 2
- Role 5 is played by actor 2
- Role 6 is played by actor 4

### 2 Show that the casting problem is in NP.

To show that the casting problem is in NP (nondeterministic polynomial time), we need to demonstrate that any given solution to the problem can be verified in polynomial time.

Here is some psudo code for a verifier for the casting problem:

```python
def verifyCasting(scenesRoles, rolesAssignedActors):
    """Input:
        scenesRoles - a list of lists, each sublist contains roles involved in that scene
        rolesAssignedActors - a list where index represents the role and value is the assigned actor (None if unassigned)

    Output:
        Returns True if the assignment is valid according to all constraints, False otherwise
    """

    # Check if every role is assigned to an actor from its allowed list
    for role, assignedActor in enumerate(rolesAssignedActors):
        if assignedActor is None or not isActorAllowedForRole(role, assignedActor):
            # If no actor is assigned or the actor is not allowed for the role, return False
            return False

    # Check for unique actors in each scene
    for scene, roles in enumerate(scenesRoles):
        actorsInScene = set()
        for role in roles:
            assignedActor = rolesAssignedActors[role]
            if assignedActor in actorsInScene:
                # Actor is repeated in the same scene, which is not allowed
                return False
            actorsInScene.add(assignedActor)

    # Check that divas p1 and p2 do not appear in the same scene
    diva1Scenes = actor_in_scenes(scenesRoles, rolesAssignedActors, diva1)
    diva2Scenes = actor_in_scenes(scenesRoles, rolesAssignedActors, diva2)
    for scene in diva1Scenes:
        if scene in diva2Scenes:
            # Divas appear in the same scene, which violates their constraint
            return False

    return True

# Helper function to check if an actor is allowed for a role
Function isActorAllowedForRole(role, actor):
    # Check if the actor is in the allowed list for the role
    return actor in roleToAllowedActors[role]

# Functions to support scene and role mappings
Function actor_in_scenes(scenesRoles, rolesAssignedActors, actor):
    roles = actor_plays_roles(rolesAssignedActors, actor)
    scenes = set()
    for role in roles:
        roleScenes = role_in_scenes(scenesRoles, role)
        scenes.update(roleScenes)
    return scenes

Function actor_plays_roles(rolesAssignedActors, actor):
    return [role for role, assignedActor in enumerate(rolesAssignedActors) if assignedActor == actor]

Function role_in_scenes(scenesRoles, role):
    return [scene for scene, roles in enumerate(scenesRoles) if role in roles]

```

Overall, every step of the algorithm involves iterating through lists of roles, actors, or scenes at most a few times. None of the steps require more than polynomial time with respect to the input size (i.e., the number of roles, actors, and scenes). Thus, the verification process runs in polynomial time, making it feasible for even reasonably large instances of the problem, aligning with the requirements for a problem to be in NP, as verification of a given solution is efficient.

### 3 Suppose we want to modify the no-instance above into a yes-instance, by adding a few actors. How many actors do we need to add in this case?  (Assume every actor we add can play every role.)
We only need to add one more actor for role three. That new actor don't have to be able to play any other role as it is only need to play role three so actor 1 donät have to play it.

    5 
    5
    3

    3 - 1 2 3 
    2 - 2 3 
    3 - 1 3 4
    1 - 2 
    3 - 1 2 3 

    2 - 1 2
    2 - 1 2
    3 - 1 3 4
    2 - 3 5
    3 - 2 3 5

### 4 Which is the smallest possible production that satisfies all input constraints for the casting problem and is possible to stage (i.e., the smallest possible YES-instance)? Specify the input for this production.

#### The smallest possible production
    2
    1
    2

    1 - 1
    1 - 2

    2 - 1 2

#### The smallest possible production where actors 1 and 2 are not in the same scene
    2
    2
    3

    1 - 1
    1 - 2
    1 - 3

    2 - 1 3
    2 - 2 3

### 5 Imagine an instance with 3 or more roles, where the roles can be divided into two groups, such that each role never occurs in a scene together with another role from the same group.  In other words the scenes and roles has a structure similar to a bipartite graph. Suppose further that every actor can play every role.  How many actors will be sufficient in this case?

If the roles are divided into two groups where roles from the same group never appear together, the situation mirrors a bipartite graph. In this case, you only need 2 actors, one for each group of roles, since no two roles from the same group share a scene.

### 6 Suppose that an instance A contains a scene with the roles 4, 7 and 12, while the instance B has three scenes with the roles 4 and 7, 7 and 12, 4 and 12. If all other constraints are identical between the instances, will the solutions be identical? Why/why not?

The solutions will be identical.
The reasons is as follows. From the constraint that no actor can play more than one role in a scene we can derive more constraints on which roles can't be played by the same actor.
In instance A then scene with roles 4, 7 and 12 gives us the constraints:
- The roles 4 and 7 can't be played by the same actor
- The roles 7 and 12 can't be played by the same actor
- The roles 4 and 12 can't be played by the same actor

And in instance B the scenes with roles 4 and 7, 7 and 12, 4 and 12 gives us the constraints:
- The roles 4 and 7 can't be played by the same actor
- The roles 7 and 12 can't be played by the same actor
- The roles 4 and 12 can't be played by the same actor

The constraints are the same in both instances and therefore the solutions will be identical.