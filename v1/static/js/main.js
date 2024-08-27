async function loadProjects() {
    const response = await fetch('/projects');
    const projects = await response.json();
    const projectsDiv = document.getElementById('projects');
    projectsDiv.innerHTML = '';
    projects.forEach(project => {
        const div = document.createElement('div');
        div.textContent = project.name;
        projectsDiv.appendChild(div);
    });
}

document.getElementById('createProject').addEventListener('click', async () => {
    const projectName = prompt('Enter project name:');
    if (projectName) {
        await fetch('/projects', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: projectName })
        });
        loadProjects();
    }
});

loadProjects();
