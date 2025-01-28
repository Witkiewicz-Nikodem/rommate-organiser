document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById("JoinGroupForm");
    form.addEventListener('submit', async (event) =>{
        event.preventDefault();

        
        const formData = new FormData(form);
        const data = Object.fromEntries(formData.entries());
        
        try{
            const response = await fetch(
                'https://rust-app-production.up.railway.app/group/join_group',{
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data),
            });

            if(response.ok){
                alert('Join Group succesful!');
            } else{
                const errorData = await response.json();
                alert(`Create group failed: ${errorData.message || 'Unknown error'}`);
            }
        } catch (error){
            console.error("Error during join group: ", error);
            alert('An error occured. Please try again later.');
        };
    });
})