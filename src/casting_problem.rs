use regex::Regex;
use std::collections::HashSet;


fn has_common_element<I, J>(iter1: I, iter2: J) -> bool
where
    I: IntoIterator<Item = usize>,
    J: IntoIterator<Item = usize>,
{
    let set: HashSet<usize> = iter1.into_iter().collect();
    iter2.into_iter().any(|item| set.contains(&item))
}


fn actor_plays_roles<'a>(
    roles_assigned_actors: &'a Vec<Option<usize>>,
    actor: usize,
) -> impl Iterator<Item = usize> + 'a {
    roles_assigned_actors
        .iter()
        .enumerate()
        .filter_map(move |(role, &assigned_actor)| {
            if let Some(assigned_actor) = assigned_actor {
                if assigned_actor == actor {
                    return Some(role);
                }
            }
            None
        })
}


fn role_in_scenes<'a>(
    scenes_roles: &'a Vec<Vec<usize>>,
    role: usize,
) -> impl Iterator<Item = usize> + 'a {
    scenes_roles
        .iter()
        .enumerate()
        .filter_map(move |(scene, roles)| {
            if roles.contains(&role) {
                return Some(scene);
            }
            None
        })
}


fn actor_in_scenes<'a>(
    scenes_roles: &'a Vec<Vec<usize>>,
    roles_assigned_actors: &'a Vec<Option<usize>>,
    actor: usize,
) -> impl Iterator<Item = usize> + 'a {
    actor_plays_roles(roles_assigned_actors, actor)
        .into_iter()
        .flat_map(|role| {
            let role_scenes = role_in_scenes(scenes_roles, role);

            role_scenes
        })
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct RoleActor {
    role: usize,
    actor: usize,
}


fn get_actors_for_role<'a>(
    roles_potential_actors: &'a Vec<Vec<usize>>,
    scenes_roles: &'a Vec<Vec<usize>>,
    roles_assigned_actors: &'a Vec<Option<usize>>,
    role: usize,
) -> impl Iterator<Item = usize> + 'a {
    if let Some(actor) = roles_assigned_actors[role] {
        panic!(
            "Role {} is already assigned to actor {}",
            role + 1,
            actor + 1
        );
    }

    roles_potential_actors[role]
        .iter()
        .filter_map(move |&actor| {
            let role_scenes = role_in_scenes(scenes_roles, role);
            let actor_scenes = actor_in_scenes(scenes_roles, roles_assigned_actors, actor);

            if has_common_element(role_scenes, actor_scenes) {
                return None;
            } else {
                return Some(actor);
            }
        })
}


fn get_options<'a>(
    roles_potential_actors: &'a Vec<Vec<usize>>,
    scenes_roles: &'a Vec<Vec<usize>>,
    roles_assigned_actors: &'a Vec<Option<usize>>,
) -> impl Iterator<Item = RoleActor> + 'a {
    roles_assigned_actors
        .iter()
        .enumerate()
        .filter_map(|(role, &actor)| {
            if actor.is_none() {
                return Some(
                    get_actors_for_role(
                        roles_potential_actors,
                        scenes_roles,
                        roles_assigned_actors,
                        role,
                    )
                    .map(move |actor| RoleActor { role, actor }),
                );
            } else {
                return None;
            }
        })
        .flatten()
}


fn explore_options_recursive(
    roles_potential_actors: &Vec<Vec<usize>>,
    scenes_roles: &Vec<Vec<usize>>,
    roles_assigned_actors: &Vec<Option<usize>>,
) -> Vec<Vec<usize>> {
    let options = get_options(roles_potential_actors, scenes_roles, roles_assigned_actors);
    let mut all_valid_assignments = Vec::new();

    for option in options {

        let mut new_roles_assigned_actors = roles_assigned_actors.clone();
        new_roles_assigned_actors[option.role] = Some(option.actor);


        if new_roles_assigned_actors.iter().all(|a| a.is_some()) {
            all_valid_assignments.push(
                new_roles_assigned_actors
                    .iter()
                    .map(|actor| actor.unwrap())
                    .collect(),
            );
        } else {
            let mut results_from_recursion = explore_options_recursive(
                roles_potential_actors,
                scenes_roles,
                &new_roles_assigned_actors,
            );

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


fn apply_actor_filter(solution: &Vec<usize>, scenes_roles: &Vec<Vec<usize>>) -> bool {
    let roles_assigned_actors = solution.iter().map(|&a| Some(a)).collect::<Vec<_>>();
    let scenes_1 = actor_in_scenes(scenes_roles, &roles_assigned_actors, 0);
    let scenes_2 = actor_in_scenes(scenes_roles, &roles_assigned_actors, 1);
    return !has_common_element(scenes_1, scenes_2);
}


pub fn run(input: &str) {
    let (roles_potential_actors, scenes_roles) = parse_input(input);

    let roles_assigned_actors = vec![None; roles_potential_actors.len()];

    let all_solution = explore_options_recursive(
        &roles_potential_actors,
        &scenes_roles,
        &roles_assigned_actors,
    );

    let solutions = all_solution
        .iter()
        .filter(|solution| apply_actor_filter(solution, &scenes_roles))
        .collect::<Vec<_>>();

    if solutions.is_empty() {
        println!("No solution found");
    } else {
        println!("Found {} solutions", solutions.len());

        solutions
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(role, actor)| {
                println!("Role {} is played by actor {}", role + 1, actor + 1);
            });
    }
}

pub fn reduce_to_graph_coloring(input: &str) {
    let (roles_potential_actors, scenes_roles) = parse_input(input);

    let n_vertices = roles_potential_actors.len();
    let n_colors = roles_potential_actors.iter().flat_map(|a| a.iter()).max().unwrap() + 1;
    let mut edges: Vec<(usize, usize)> = vec![];

    for scene in scenes_roles {
        for i in 0..scene.len() {
            for j in i + 1..scene.len() {
                edges.push((scene[i], scene[j]));
            }
        }
    }

    println!("{}", n_vertices);
    println!("{}", edges.len());
    println!("{}", n_colors);

    for (from, to) in edges {
        println!("{} {}", from + 1, to + 1);
    }
}
