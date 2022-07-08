# Projet Rust 4eme annee : Groupe 7 #


# ° Sérialisation

    Mise en place des structures et de la sérialisation des messages :
        -  Pair programming (Lucas, Kélyan) 
        -  Mise en place d'un serveur maison pour tester le sérialisation et la communication client/serveur


# ° Résolution des challanges :
##### HashCash :
  - Ecriture du pseudo code en pair programming (Halimatou, Lucas, Kélyan)


    complexite = 9
    message = "hello"

    cestbon = false
    
    caractere_en_plus = 0000000000000000
    res;
    hash;
    
    while(!cestbon){
    
        res = caractere_en_plus + message
        
        hash = md5(res)
        zero = count_zero(hash)
        
        if (zero * 8 >= complexite ){
            cestbon = true
            //(res, hash);
        }
    
        caractere_en_plus + 1hexa
    }
    return (res, hash)



### Réalisation de l'algo : Kélyan + Lucas pour la partie incrémentation de l'hexa
  - Incrémentation de l'hexadécimal : 
    - `fn increase(mut vec : Vec<u8>) -> Vec<u8>{}`
    - Refacto :  Conversion en u64


##### Recover Secret : Halimatou 



# ° Communication avec le serveur : Kélyan
* Etapes de réalisation :
    - Mettre en dur les étapes d'envoie de messages
    - Inclure dans des fonctions
    - Création de fonction de parsing pour convertir les messages reçu en structures
    - Interpretation des messages reçu pour les challenges (Pair Programming Lucas + Kélyan)


# ° Test du projet  : 
    Lucas












