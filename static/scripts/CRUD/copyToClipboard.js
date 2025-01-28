function copyToClipboard(){
    const copyCodeContainer = document.getElementById('copyCode');
    const data = copyCodeContainer.innerHTML; 
    const code = data.split("join group code: ")[1];
    if(code != " "){
        navigator.clipboard.writeText(code);
        alert("Join code is in your clipboard now")
    }
}