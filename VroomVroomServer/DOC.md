# TrackMania 

TrackMania est une série de jeux vidéos de course développés par Nadéo et Firebrand Games entre 2003 et 2020. La dernière édition du jeu, sortie en 2020, est proposée parmi les jeux de l'Insalan XVI.



## Procédure utilisation et configuration

### Lancement / Arret 
Les dockers TM sont sur SunFlower : **172.16.1.7** \
Lancement : _docker-compose -p tm_server_truc -f docker-compose.yaml up -d_ \
Arret : _docker-compose -p tm_server_truc -f docker-compose.yaml down -v_ \
**Attention** à changer “truc” pour chaque serveur (genre time, cup1 cup2, …) \
**Attention** à être dans le bon dossier \
Le nom qui apparaît dans TrackMania peut être défini dans `docker-compose.yaml`

### Parametre de jeu
Dans `compose/maps/MatchSettings/cfg_tracklist.xml` sous _<script_settings>_ on peut changer les paramètres des matchs du serveur.
> Exemple : _\<setting name="S_AllowRespawn" type="boolean" value="1"/>_

La liste complete des settings, leur default-value, et leur description: [ici](https:_doc.maniaplanet.com/dedicated-server/references/settings-list-for-nadeo-gamemodes)

D’autres paramètres peuvent être configurés en passant par PyPlanet, notamment pour définir les admins, interdire le chat, autoriser les votes pour skip : dans `apps.yaml` et `base.yaml` sous `compose/pyplanet/settings/`. Les admins sont defini sous “OWNER: default:” grâce à leur login Trackmania

Dans `compose/cfg_server.xml`, on peut gérer des paramètres généraux du serveur, notamment le nombre max de joueurs sur ce serveur.


### Map pool 
Dans `compose/maps/MatchSettings/cfg_tracklist.xml` on peut indiquer les maps : sous _\<map>_, mettre _\<file>aDirectoy/aTMmap.Gbx\</file>_. Soyez sûr d’avoir mis le fichier Gbx correspondant à la map dans `compose/maps/aDirectoy`.
> Exemple : _\<map><file>Campaigns/Simple LoL #13.Map.Gbx</file></map>_ dans le `cfg_tracklist.xml` avec le fichier “Simple LoL #13.Map.Gbx” dans `compose/maps/Campaigns`.


### Configuration des ports 
Faut changer les ports dans `compose/cfg_server.xml` _<server_port>_ et `compose/docker-compose.yaml` _ports_ et mettre les **MÊME** ports (évidemment) **SUPÉRIEUR** à 2350:
  * Time     -> 2350 
  * Cup1     -> 2351 
  * Cup2     -> 2352
  * Cup3     -> 2353
  * Cup4     -> 2354
  * Cup5     -> 2355
  * Cup6     -> 2356
  * Cup7     -> 2357
  * Cup8     -> 2358
  * Train (TA) -> 2359


### Sources 
- [Pyplanet](https://pypla.net/en/latest/)
- [Maniaplanet](https://doc.maniaplanet.com/)
- [GitHub d'origine](https://github.com/Harha/trackmania-server-docker/)




## Troubleshooting 

#### Serveurs Invisibles 

Cette section concerne le problème spécifique qui impacte certains clients mais pas d'autres chez des gens branchés sur le même switch, dans le bon VLAN, et une fois qu'il a été vérifié (par `ipconfig` et `ping`) que l'on est dans le _bon VLAN_ et que l'on peut _toucher le serveur_.

Il arrive, parce que [la stack](https://social.technet.microsoft.com/Forums/ie/en-US/99395fe0-eb8f-49d1-853e-e4677a0b70e2/default-interface-for-broadcast?forum=itprovistanetworking) [réseau]([https://github.com/dechamps/WinIPBroadcast#rationale) [de Windows](https://serverfault.com/questions/72112/how-to-alter-the-global-broadcast-address-255-255-255-255-behavior-on-windows) [pue de l'aiselle](https://stackoverflow.com/questions/62970309/why-doesnt-windows-desktop-broadcast-udp) (ya 4 liens), que les paquets envoyés lorsqu'un client scanne le VLAN TM pour trouver le serveur ne partent pas par l'Ethernet, malgré le fait que tout le reste du trafic du PC y passe. En effet, au scan, le client TM envoie un paquet UDP à `172.17.X.255` (si le VLAN TM est en `172.17.X.0/24`) pour tous les ports de 3500 (premier port, par défaut, pour le serveur) jusqu'à au moins 3600. Si quelque chose lui répond d'une façon qu'il aime, avec les bonnes infos, c'est un serveur trouvé. Cependant, si le paquet est _envoyé sur le wifi_ bah ça merde.

Donc on doit aller dire à Windows de privilégier l'interface filaire quoi qu'il arrive. Le choix de l'interface "primaire" (celle vers laquelle les paquets de broadcast sont envoyés), est fait en regardant la plus petite [métrique d'interface](https://docs.microsoft.com/en-us/troubleshoot/windows-server/networking/automatic-metric-for-ipv4-routes).

Le fix est fait en 9 étapes, mais c'est un clickodrome, vu qu'on doit passer par la configuration de Windows (les panneaux de configuration legacy) :

1. Ouvrir les paramètres réseau (clic gauche sur l’icône réseau sur la barre des tâches)
2. Ouvrir le centre réseau et partage sur le côté
3. Aller dans les détails de la connexion Ethernet
4. Rentrer dans les paramètres de cette connexion (le joueur aura probablement à rentrer un mdp admin sur son PC)
5. Dans les modules proposés, sélectionner “Protocole Internet Version 4 (TCP/IPv4)”, puis appuyer sur “Propriétés”
6. Aller dans les paramètres avancés
7. Décocher la “Métrique automatique”
8. Mettre comme valeur 1.
9. Tout refermer en faisant “OK”

Il faudra très probablement enlever ce fix aux joueurs une fois la LAN finie, mais nous ne savons à l'heure actuelle (pré Insalan XVI) pas encore.