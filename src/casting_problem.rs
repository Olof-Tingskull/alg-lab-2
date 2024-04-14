use regex::Regex;

fn actor_plays_roles(roles_assigned_actors: &Vec<Option<usize>>, actor: usize) -> Vec<usize> {
    roles_assigned_actors
        .iter()
        .enumerate()
        .filter_map(|(role, &assigned_actor)| {
            if let Some(assigned_actor) = assigned_actor {
                if assigned_actor == actor {
                    return Some(role);
                }
            }
            None
        })
        .collect()
}

fn role_in_scenes(scenes_roles: &Vec<Vec<usize>>, role: usize) -> Vec<usize> {
    scenes_roles
        .iter()
        .enumerate()
        .filter_map(|(scene, roles)| {
            if roles.contains(&role) {
                return Some(scene);
            }
            None
        })
        .collect()
}

fn actor_in_scenes(
    scenes_roles: &Vec<Vec<usize>>,
    roles_assigned_actors: &Vec<Option<usize>>,
    actor: usize,
) -> Vec<usize> {
    let roles = actor_plays_roles(roles_assigned_actors, actor);
    roles
        .iter()
        .flat_map(|&role| {
            let role_scenes = role_in_scenes(scenes_roles, role);

            role_scenes
        })
        .collect()
}

#[derive(Debug)]
struct RoleActor {
    role: usize,
    actor: usize,
}

fn get_actors_for_role(
    roles_potential_actors: &Vec<Vec<usize>>,
    scenes_roles: &Vec<Vec<usize>>,
    roles_assigned_actors: &Vec<Option<usize>>,
    role: usize,
) -> Vec<usize> {
    if let Some(actor) = roles_assigned_actors[role] {
        panic!(
            "Role {} is already assigned to actor {}",
            role + 1,
            actor + 1
        );
    }

    roles_potential_actors[role]
        .iter()
        .filter_map(|&actor| {
            let role_scenes = role_in_scenes(scenes_roles, role);
            let actor_scenes = actor_in_scenes(scenes_roles, roles_assigned_actors, actor);

            if role_scenes.iter().any(|scene| actor_scenes.contains(scene)) {
                return None;
            }

            Some(actor)
        })
        .collect()
}

fn get_options(
    roles_potential_actors: &Vec<Vec<usize>>,
    scenes_roles: &Vec<Vec<usize>>,
    roles_assigned_actors: &Vec<Option<usize>>,
) -> Vec<RoleActor> {
    roles_assigned_actors
        .iter()
        .enumerate()
        .filter(|(_role, &actor)| actor.is_none())
        .flat_map(|(role, _actor)| {
            let m = get_actors_for_role(
                roles_potential_actors,
                scenes_roles,
                &roles_assigned_actors,
                role,
            )
            .iter()
            .map(|&actor| RoleActor { role, actor })
            .collect::<Vec<_>>();

            m
        })
        .collect()
}

fn explore_options_recursive(
    roles_potential_actors: &Vec<Vec<usize>>,
    scenes_roles: &Vec<Vec<usize>>,
    roles_assigned_actors: &Vec<Option<usize>>,
) -> Vec<Vec<usize>> {
    let options = get_options(roles_potential_actors, scenes_roles, roles_assigned_actors);

    for option in &options {
        //println!("Role {} can be played by actor {}", option.role, option.actor);
    }  
    //panic!("");

    // Initialize a vector to store all valid assignments
    let mut all_valid_assignments = Vec::new();

    // If there are no options, return the empty list of assignments (base case of recursion)
    if options.is_empty() {
        return all_valid_assignments;
    }

    for option in options {
        let mut new_roles_assigned_actors = roles_assigned_actors.clone();
        new_roles_assigned_actors[option.role] = Some(option.actor);

        // Check if all roles have been assigned an actor
        if new_roles_assigned_actors
            .iter()
            .all(|actor| actor.is_some())
        {
            // Collect this valid assignment
            all_valid_assignments.push(
                new_roles_assigned_actors
                    .iter()
                    .map(|actor| actor.unwrap())
                    .collect(),
            );
        } else {
            // Otherwise, continue exploring recursively
            let mut results_from_recursion = explore_options_recursive(
                roles_potential_actors,
                scenes_roles,
                &new_roles_assigned_actors,
            );
            // Append all results from further recursion into the main list
            all_valid_assignments.append(&mut results_from_recursion);
        }
    }

    all_valid_assignments
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let re = Regex::new(r"\d+").unwrap();

    let mut numbers: Vec<_> = re
        .find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect();

    let n = numbers.remove(0);
    let s = numbers.remove(0);
    let _k = numbers.remove(0);

    let mut left_in_group = 0;
    let groups: Vec<Vec<usize>> = numbers.iter().fold(vec![], |mut acc, &num| {
        if left_in_group == 0 {
            left_in_group = num;
            acc.push(vec![]);
        } else {
            acc.last_mut().unwrap().push(num - 1);
            left_in_group -= 1;
        }
        acc
    });

    return (
        groups.iter().take(n).map(|group| group.clone()).collect(),
        groups
            .iter()
            .skip(n)
            .take(s)
            .map(|group| group.clone())
            .collect(),
    );
}

pub fn run(input: &str) {
    let (roles_potential_actors, scenes_roles) = parse_input(input);

    
    let all_solutions = explore_options_recursive(
        &roles_potential_actors,
        &scenes_roles,
        &vec![None; roles_potential_actors.len()],
    );


    if all_solutions.is_empty() {
        println!("No solution found");
    } else {
        println!("Found {} solutions", all_solutions.len());

        all_solutions
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(role, actor)| {
                println!("Role {} is played by actor {}", role + 1, actor + 1);
            });
    }
}
