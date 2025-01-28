document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById("CreateGroupForm");
    form.addEventListener('submit', async (event) =>{
        event.preventDefault();

        
        const formData = new FormData(form);
        const data = Object.fromEntries(formData.entries());
        
        try{
            const response = await fetch(
                'https://rust-app-production.up.railway.app/group',{
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data),
            });

            if(response.ok){
                alert('Creat Group succesful!');
                window.location.href = "/logged_in/create_group"
            } else{
                const errorData = await response.json();
                alert(`Create group failed: ${errorData.message || 'Unknown error'}`);
            }
        } catch (error){
            console.error("Error during create group: ", error);
            alert('An error occured. Please try again later.');
        };
    });
})