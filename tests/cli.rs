mod common;

use clap::Parser;
use common::establish_test_connection;
use on_a_roll::{
    cli::{run_cli, Cli},
    models::Project,
};

#[test]
fn test_create_project_via_cli() {
    let mut conn = establish_test_connection();

    let args = vec!["roll", "project", "add", "Integration Test Project"];

    let cli = Cli::parse_from(args);
    run_cli(cli, &mut conn);

    let projects = Project::list(&mut conn).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].title, "Integration Test Project");
}

#[test]
fn test_update_project_via_cli() {
    let mut conn = establish_test_connection();

    let create_args = vec!["roll", "project", "add", "Initial Project"];
    let cli = Cli::parse_from(create_args);
    run_cli(cli, &mut conn);

    let project = Project::list(&mut conn).unwrap()[0].clone();

    let project_id_string = project.id.to_string();
    let update_args = vec![
        "roll",
        "project",
        "update",
        &project_id_string,
        "--title",
        "Updated Project",
    ];
    let cli = Cli::parse_from(update_args);
    run_cli(cli, &mut conn);

    let updated_project = Project::find(&mut conn, project.id).unwrap();
    assert_eq!(updated_project.title, "Updated Project");
}

#[test]
fn test_delete_project_via_cli() {
    let mut conn = establish_test_connection();

    let create_args = vec!["roll", "project", "add", "Project to Delete"];
    let cli = Cli::parse_from(create_args);
    run_cli(cli, &mut conn);

    let project = Project::list(&mut conn).unwrap()[0].clone();
    let project_id_string = project.id.to_string();
    let delete_args = vec!["roll", "project", "delete", &project_id_string];
    let cli = Cli::parse_from(delete_args);
    run_cli(cli, &mut conn);

    let projects = Project::list(&mut conn).unwrap();
    assert!(projects.is_empty());
}

#[test]
fn test_read_project_via_cli() {
    let mut conn = establish_test_connection();

    let create_args = vec!["roll", "project", "add", "Project to Read"];
    let cli = Cli::parse_from(create_args);
    run_cli(cli, &mut conn);

    let project = Project::list(&mut conn).unwrap()[0].clone();
    let project_id_string = project.id.to_string();
    let read_args = vec!["roll", "project", "read", &project_id_string];
    let cli = Cli::parse_from(read_args);
    run_cli(cli, &mut conn);

    let found_project = Project::find(&mut conn, project.id).unwrap();
    assert_eq!(found_project.title, "Project to Read");
}

#[test]
fn test_list_projects_via_cli() {
    let mut conn = establish_test_connection();

    let create_args_1 = vec!["roll", "project", "add", "Project 1"];
    let cli = Cli::parse_from(create_args_1);
    run_cli(cli, &mut conn);

    let create_args_2 = vec!["roll", "project", "add", "Project 2"];
    let cli = Cli::parse_from(create_args_2);
    run_cli(cli, &mut conn);

    let list_args = vec!["roll", "project", "list"];
    let cli = Cli::parse_from(list_args);
    run_cli(cli, &mut conn);

    let projects = Project::list(&mut conn).unwrap();
    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].title, "Project 1");
    assert_eq!(projects[1].title, "Project 2");
}
