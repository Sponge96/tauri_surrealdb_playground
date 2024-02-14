const { invoke } = window.__TAURI__.tauri;

let currentView = null;

document.addEventListener("DOMContentLoaded", function () {
    // Fetch projects and populate the project-list div
    showView();
    fetchProjects();
});

function showView(viewName = 'projectView') {
    if (currentView) {
        const currentViewElement = document.getElementById(currentView);
        if (currentViewElement) {
            currentViewElement.style.display = 'none';
        }
    }

    const newViewElement = document.getElementById(viewName);
    if (newViewElement) {
        newViewElement.style.display = 'block';
        currentView = viewName;
    }
}


async function fetchProjects() {
    try {
        const response = await invoke("list_projects"); // Update the URL accordingly
        if (response.error) {
            console.error("Error fetching projects:", response.error.message);
            return;
        }

        const projects = response.result.data;
        displayProjects(projects);
    } catch (error) {
        console.error("Error fetching projects:", error.message);
    }
}

function displayProjects(projects) {
    const projectListDiv = document.getElementById("project-list");

    // Clear existing content
    projectListDiv.innerHTML = "";

    projects.forEach((project) => {
        const projectContainer = document.createElement("div");
        projectContainer.classList.add("project");

        const nameElement = document.createElement("span");
        nameElement.textContent = project.name;
        nameElement.classList.add("project-name");
        projectContainer.appendChild(nameElement);

        const programsList = document.createElement("div");
        programsList.classList.add("program-list");

        project.programs.forEach((program, index) => {
            const programTag = document.createElement("span");
            programTag.textContent = program;
            programTag.classList.add("program-tag");
            programsList.appendChild(programTag);
        });

        projectContainer.appendChild(programsList);
        projectListDiv.appendChild(projectContainer);
    });
}
