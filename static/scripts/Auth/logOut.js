document.addEventListener('DOMContentLoaded', () => {
  const form = document.getElementById('logoutbutton');
  form.addEventListener('click', async (event) => {
    event.preventDefault(); 

    try {
      const response = await fetch('https://rust-app-production.up.railway.app/log_out', {
        method: 'DELETE',
      });

      // Obsługa odpowiedzi
      if (response.ok) {
        alert('Logout Successful!');
        window.location.pathname = "/logged_out/home"
      } else {
        const errorData = await response.json();
        alert(`Logout failed: ${errorData.message || 'Unknown error'}`);
      }

    } catch (error) {
      console.error('Error during Logout:', error);
      alert('An error occurred. Please try again later.');
    }
  });
});
