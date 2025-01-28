document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('changeGroupNameForm');
    
    form.addEventListener('submit', async (event) => {
      event.preventDefault(); 
  
      const formData = new FormData(form);
      const data = Object.fromEntries(formData.entries()); 
      alert(data)
  
      try {
        const response = await fetch('https://rust-app-production.up.railway.app/group', {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(data),
        });
  
        if (response.ok) {
          window.location.href = "/logged_in/manage_owned_groups"
          alert('change completed');
        } else {
          const errorData = await response.json();
          alert(`Register failed: ${errorData.message || 'Unknown error'}`);
        }
      } catch (error) {
        console.error('Error during register:', error);
        alert('An error occurred. Please try again later.');
      }
    });
  });