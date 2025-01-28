document.addEventListener('DOMContentLoaded', () => {
  const form = document.getElementById('registrationForm');
  
  form.addEventListener('submit', async (event) => {
    event.preventDefault(); 


    const formData = new FormData(form);
    const data = Object.fromEntries(formData.entries()); 

    try {
      const response = await fetch('https://rust-app-production.up.railway.app/user', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      });

      if (response.ok) {
        alert('Register Successful!');
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