function update_class(sectionId,class_for_chosen, class_for_rest) {
    // Get all sections
    const to_remove = document.querySelectorAll('.'+ class_for_chosen);
    to_remove.forEach(element => {
        element.classList.remove(class_for_chosen);
        element.classList.add(class_for_rest);
    });
  
    const chosen_element = document.getElementById(sectionId)
    chosen_element.classList.remove(class_for_rest);
    chosen_element.classList.add(class_for_chosen);
  }
  