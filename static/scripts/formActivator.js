function showdistinct(sectionId, class_name) {
  const activeSection = document.getElementById(sectionId);
  if (activeSection) {
    activeSection.classList.remove(class_name);
  }
}

function hidedistinct(sectionId, class_name) {
  const activeSection = document.getElementById(sectionId);
  if (activeSection) {
    activeSection.classList.add(class_name);
  }
}