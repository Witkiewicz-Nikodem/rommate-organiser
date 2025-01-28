export {showSections, hideSections}

function showSections() {
    const logged_in_user_sections = [
        'logout',
        'groupsDropDown',
        'expenses'
    ];

    logged_in_user_sections.forEach(element => {
        doshowSection(element);
    });
}

function doshowSection(sectionId) {
    const sth = document.getElementById(sectionId);

    if (sth) {
        sth.classList.remove('out');
        sth.classList.add('in');
    } else {
        console.warn(`Section with ID '${sectionId}' not found.`);
    }
}


function hideSections() {
    const logged_in_user_sections = [
        'logout',
        'groupsDropDown',
        'expenses'
    ];

    logged_in_user_sections.forEach(element => {
        dohideSection(element);
    });
}

function dohideSection(sectionId) {
    const sth = document.getElementById(sectionId);

    if (sth) {
        sth.classList.remove('in');
        sth.classList.add('out');
    } else {
        console.warn(`Section with ID '${sectionId}' not found.`);
    }
}