-- Your SQL goes here
-- Your SQL goes here
-- Your SQL goes here
-- Dodanie rekordów do tabeli HTML
INSERT INTO "HTML" (name, element) VALUES
('logged_in_manage_groups_head', 
'<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap" rel="stylesheet">
  <title>Roommates Organiser</title>
  <link rel="stylesheet" href="/static/css/base-styles.css">
  <link rel="stylesheet" href="/static/css/logo.css">
  <link rel="stylesheet" href="/static/css/genericButton.css">
  <link rel="stylesheet" href="/static/css/genericContent.css">
  <link rel="stylesheet" href="/static/css/drop-down.css">
  <link rel="stylesheet" href="/static/css/footer.css">
  <link rel="stylesheet" href="/static/css/Mygroup/groupButton.css">
  <link rel="stylesheet" href="/static/css/ManageOwnegGroups.css/buttons.css">
  <link rel="stylesheet" href="/static/css/ManageOwnegGroups.css/Formvisibility.css"> 
  <link rel="stylesheet" href="/static/css/authForm.css">
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>'),
('logged_in_manage_groups_individual','    
    <section id="ManageOwnedGroups" class="content DomPage">
      <h2> Manage Owned Groups </h2>               
        <div id="ManageGroupButtons" class="DomButtons"></div>
        <button id="copyCode" onclick="copyToClipboard()" class="Managebutton">join group code:  </button>
        <button id="changeGroupName" onclick="showSection(''changeform'',''Form'')" class="Managebutton">change group Name</button>
        <button id="deleteGroup" onclick="showSection(''deleteform'',''Form'')" class="Managebutton">Delete group</button>
        
        <div id = changeform class="genericContent Form">
          <form class="authForm " id="changeGroupNameForm">      
          <div class="form-group">
            <label for="old_name">provide group name to change</label>
            <input type="text" id="name-create-group" name="old_name" required>
            <label for="new_name">provide new name</label>
            <input type="text" id="name-create-group" name="new_name" required>
          </div>            
            <button class="genericButton" type="submit" onclick="hidedistinct(''changeform'', ''none'');">Submit Change</button>
          </form>  
        </div> 

        <div id = deleteform class="genericContent Form">
          <form class="authForm " id="deleteGroupNameForm">      
          <div class="form-group">
            <label for="name">provide group name to delete</label>
            <input type="text" id="name-create-group" name="name" required>
          </div>            
            <button class="genericButton" type="submit" onclick="hidedistinct(''deleteform'', ''none'');">Delete group</button>
          </form>  
        </div> 
    </section>          
  </section>'),
('logged_in_manage_groups_scripts',
'<script src="/static/scripts/Auth/logOut.js"></script>
<script src="/static/scripts/CRUD/copyToClipboard.js"></script>
<script src="/static/scripts/CRUD/delete_group.js"></script>
<script src="/static/scripts/CRUD/update_group.js"></script>
<script src="/static/scripts/sectionActivator.js"></script>
')


  