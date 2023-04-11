function Verification() {
	var nom = document.getElementById("nom").value;
	var prenom = document.getElementById("prenom").value;
	var email = document.getElementById("email").value;
	if (nom== ""){
		alert("Veuillez rentrer votre nom");
	}
	else if (prenom== ""){
		alert("Veuillez rentrer votre prenom");
	}	
	else if (email== ""){
		alert("Veuillez rentrer votre adresse email");
	}
}