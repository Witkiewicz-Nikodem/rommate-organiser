document.addEventListener('DOMContentLoaded', () => {
  const form = document.getElementById('logInForm');
  
  form.addEventListener('submit', async (event) => {
    event.preventDefault(); 

    const formData = new FormData(form);
    const data = Object.fromEntries(formData.entries());

    try {
      const response = await fetch('https://rust-app-production.up.railway.app/log_in', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      });

      if (response.ok) {
        alert('Login Successful!');
      } else {
        const errorData = await response.json();
        alert(`Login failed: ${errorData.message || 'Unknown error'}`);
      }
      window.location.href = '../logged_in/home';
    } catch (error) {
      console.error('Error during login:', error);
      alert('An error occurred. Please try again later.');
    }
  });
});
