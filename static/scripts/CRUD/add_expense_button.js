document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById("add_expense_form");
    form.addEventListener('submit', async (event) =>{
        event.preventDefault();
        
        const formData = new FormData(form);
        const data = Object.fromEntries(formData.entries());

        const pathSegments = window.location.pathname.split("/"); 
        const lastSegment = pathSegments.filter(segment => segment !== "").pop(); 
        data.group_name = decodeURIComponent(lastSegment);

        try{
            const response = await fetch(
                'https://rust-app-production.up.railway.app/expense',{
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data),
            });

            if(response.ok){
                alert('add expense succesful!');
                location.reload();
            } else{
                const errorData = await response.json();
                alert(`add expense succesful!: ${errorData.message || 'Unknown error'}`);
            }
        } catch (error){
            console.error("Error during add expense succesful!: ", error);
            alert('An error occured. Please try again later.');
        };
    });
})