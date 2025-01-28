function showSection(sectionId,section_class_name) {
    // Get all sections
    const sections = document.querySelectorAll('.'+ section_class_name);
    // Hide all sections
    sections.forEach(section => {
      section.classList.remove('active');
    });
  
    // Show the selected section
    const activeSection = document.getElementById(sectionId);
    if (activeSection) {
      activeSection.classList.add('active');
    }
  }
  