# Atlas exécutable

> Source de vision maintenue et authoritative. Conversion initiale du snapshot
> `doc/Vision_Atlas_Executable_MVP1-4.docx`, conservé comme archive de la version
> 0.1. Vérifiée par conversion Pandoc le 2026-07-13, sans divergence sémantique.
> Toute évolution de la vision doit modifier ce fichier Markdown.

Document de vision — MVP 1 à 4

*Une encyclopédie exécutable d’algorithmes, conçue pour être
comprise par l’humain, exploitée par des agents et exécutée sobrement.*

Version 0.1 — 11 juillet 2026

# 1. Résumé exécutif

Atlas exécutable est un registre de connaissances et d’artefacts logiciels qui relie quatre niveaux habituellement séparés : les problèmes, les schémas algorithmiques, leurs implémentations concrètes et les observations issues de leur exécution. Son objectif n’est pas de remplacer immédiatement les langages de programmation, ni de construire un framework universel, mais de rendre les choix algorithmiques inspectables, comparables, composables et progressivement automatisables.

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<thead>
<tr>
<th><p><strong>Vision</strong></p>
<p>À terme, un humain ou un agent doit pouvoir exprimer une intention et des contraintes, obtenir plusieurs plans d’implémentation explicables, examiner leurs structures de données, coûts, effets et hypothèses, puis exécuter ou générer le plan retenu sur un runtime minimal.</p></th>
</tr>
</thead>
<tbody>
</tbody>
</table>

La première phase valide cette vision par quatre MVP cumulatifs. Elle se limite volontairement à un corpus étroit d’algorithmes sur les séquences et collections, à des compositions simples et à un seul langage d’implémentation. MIR est évalué comme couche d’exécution intermédiaire, avec RISC-V 64 comme cible de fantasy computer. Le projet conserve cependant une séparation stricte entre le modèle de connaissance et le backend d’exécution.

# 2. Problème traité

Les bibliothèques logicielles traditionnelles exposent du code et une documentation locale. Elles décrivent rarement de façon exploitable par une machine :

- le problème abstrait réellement résolu ;

- les préconditions et garanties sémantiques ;

- les effets de bord, le déterminisme et les politiques d’allocation ;

- les compromis entre temps, mémoire, localité, parallélisme et précision ;

- les conditions dans lesquelles une implémentation domine une autre ;

- la provenance des affirmations, tests, mesures et preuves ;

- les possibilités de substitution ou de composition.

Les agents IA savent produire du code plausible, mais disposent rarement d’un catalogue structuré de composants qualifiés. Ils recomposent donc souvent des solutions déjà connues, sans maîtriser exactement les hypothèses ni les conséquences architecturales. Atlas vise à leur fournir un espace de travail plus contraint et plus vérifiable.

# 3. Principes directeurs

| **Principe** | **Conséquence** |
|----|----|
| **Connaissance avant automatisation** | Le premier produit est un modèle de connaissance utile et maintenable, pas un générateur magique. |
| **Séparation des niveaux** | Problème, algorithme, implémentation et exécution observée restent des entités distinctes. |
| **Human-readable first** | Le format primaire doit rester lisible, diffable et modifiable sans outil spécialisé. |
| **Runtime minimal** | L’intelligence est concentrée dans la conception, la validation et la génération ; l’exécution doit rester prévisible. |
| **Preuves graduées** | Une propriété peut être déclarée, inférée, testée, observée ou prouvée. Ces statuts ne sont jamais confondus. |
| **Décisions réversibles** | MIR, RISC-V, SQLite ou Rust sont des choix de phase, pas des axiomes du modèle. |
| **Pas de magie silencieuse** | Toute sélection automatique doit produire les raisons, alternatives et contraintes ayant conduit au choix. |
| **Granularité utile** | Une brique doit pouvoir être substituée, mesurée et expliquée indépendamment. |

# 4. Périmètre de la première phase

Le domaine pilote porte sur les séquences et collections :

- tri, recherche, sélection, partition, filtrage et transformation ;

- réduction, regroupement, déduplication, fusion et jointure simple ;

- tableaux contigus, séquences triées, flux bornés, tables de hachage et bitsets ;

- types scalaires simples, principalement entiers 32 et 64 bits.

Sont explicitement hors périmètre jusqu’à la fin du MVP 4 :

- réseau, objets distribués, transactions et découverte de services ;

- planification générale avec effets arbitraires ;

- langage visuel généraliste ;

- hot swapping, conteneurs applicatifs et framework avec inversion de contrôle ;

- support multi-langages complet ;

- génération autonome sans validation ;

- ontologie RDF/OWL comme format d’édition primaire.

# 5. Modèle conceptuel minimal

Le modèle initial comporte quatre entités principales et des relations explicites.

**Problem —** Décrit une transformation attendue indépendamment d’une stratégie : entrées abstraites, sorties, préconditions, postconditions et variantes.

**Algorithm —** Décrit une stratégie ou famille mathématique : propriétés théoriques, invariants, complexités et hypothèses.

**Implementation —** Décrit un artefact concret : langage, signature, ABI, dépendances, effets, allocations, cible, licence et version.

**Execution —** Décrit une observation reproductible : commit, compilateur, machine, données, paramètres, temps, mémoire, traces et résultats.

# 6. Schéma de manifeste initial

Le format d’édition de référence sera textuel, versionné et validable. YAML est proposé pour le MVP, avec une normalisation stricte et une représentation interne indépendante.

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<thead>
<tr>
<th>schema_version: 0.1<br />
<br />
problem:<br />
id: sequence.sort<br />
input:<br />
sequence: Sequence&lt;T&gt;<br />
order: TotalOrder&lt;T&gt;<br />
output:<br />
result: PermutationOf&lt;input.sequence&gt;<br />
ensures:<br />
- sorted(result, order)<br />
<br />
algorithm:<br />
id: sort.merge.top_down<br />
solves: sequence.sort<br />
properties:<br />
stable: true<br />
deterministic: true<br />
in_place: false<br />
costs:<br />
time_worst: "O(n log n)"<br />
auxiliary_memory: "O(n)"<br />
<br />
implementation:<br />
id: sort.merge.rust.slice.v1<br />
language: rust<br />
entrypoint: merge_sort<br />
effects:<br />
mutates: [input.sequence]<br />
io: none<br />
blocking: false<br />
evidence:<br />
tests: [sorted, permutation, stability]</th>
</tr>
</thead>
<tbody>
</tbody>
</table>

# 7. Architecture cible de la phase

| **Composant** | **Responsabilité** |
|----|----|
| **atlas-schema** | Types du modèle, validation structurelle, versions du schéma. |
| **atlas-registry** | Stockage, recherche, relations, provenance et historique. |
| **atlas-runner** | Compilation, exécution isolée, collecte de résultats et traces. |
| **atlas-bench** | Jeux de données, mesures reproductibles, profils de machines. |
| **atlas-query** | Filtrage par contraintes, classement et justification. |
| **atlas-compose** | Composition limitée de pipelines et génération d’un plan. |
| **atlas-mir** | Adaptateur expérimental vers MIR ; aucune dépendance du modèle vers MIR. |
| **atlas-cli** | Interface de référence pour les humains et les agents. |

# 8. Feuille de route MVP 1 à 4

## MVP 1 — Registre documentaire exécutable

Objectif : prouver que le modèle de fiche peut décrire un petit corpus sans devenir disproportionné.

- Définir le schéma 0.1 et ses règles de versionnement.

- Décrire 10 problèmes, 15 algorithmes et 20 implémentations Rust.

- Stocker les manifestes dans Git ; indexer leur projection dans SQLite.

- Valider les identifiants, références, types, propriétés obligatoires et provenance.

- Fournir une CLI : list, show, validate, search et explain.

- Associer à chaque implémentation une suite de tests reproductible.

Critères de sortie :

- Un nouveau composant peut être ajouté sans modifier le code du registre.

- Au moins 90 % des propriétés obligatoires sont remplies pour le corpus pilote.

- Les mêmes manifestes reconstruisent un registre identique.

- Chaque affirmation importante indique sa provenance et son niveau de confiance.

## MVP 2 — Qualification empirique

Objectif : relier les affirmations théoriques à des observations reproductibles.

- Créer un harnais commun de compilation, tests et benchmarks.

- Définir des profils de données : tailles, distributions, duplications, ordre initial.

- Capturer machine, système, compilateur, options, commit et graine.

- Mesurer temps, mémoire maximale, allocations et volume de données parcouru lorsque possible.

- Comparer plusieurs implémentations d’un même problème.

- Produire un rapport expliquant les domaines de domination observés sans les généraliser abusivement.

Critères de sortie :

- Une exécution est rejouable à partir de son identifiant.

- Les résultats distinguent clairement théorie, déclaration et observation.

- Le système peut répondre à une requête du type : « tri stable sous une limite mémoire donnée ».

- Les mesures aberrantes ou non comparables sont signalées.

## MVP 3 — Sélection explicable et compositions linéaires

Objectif : démontrer une première forme de synthèse contrainte, limitée et compréhensible.

- Introduire des types de données structuraux minimaux et des conversions explicites.

- Construire des pipelines acycliques linéaires : parse → filter → sort → deduplicate → encode.

- Résoudre la compatibilité des entrées, sorties, préconditions et effets autorisés.

- Classer les plans selon des objectifs simples : mémoire, latence, déterminisme, allocations.

- Afficher les alternatives rejetées et les raisons de rejet.

- Générer un plan sérialisé et un programme Rust d’orchestration lisible.

Critères de sortie :

- Au moins cinq scénarios complets sont reconstruits à partir de composants du registre.

- Le code d’orchestration généré ne contient aucune logique métier cachée.

- Toute conversion ou copie de données est visible dans le plan.

- Le développeur peut forcer ou interdire une implémentation sans modifier le registre.

## MVP 4 — Backend MIR et fantasy computer RISC-V

Objectif : vérifier que le même plan peut être exécuté par un backend plus léger et plus proche d’une machine pédagogique, sans contaminer le modèle de connaissance.

- Définir une interface backend indépendante : appel, mémoire, imports, exports et traces.

- Compiler ou traduire un sous-ensemble des implémentations vers MIR.

- Exécuter le même corpus en interprétation MIR et en JIT sur les cibles disponibles.

- Valider le backend RISC-V 64 de MIR et documenter ses limites réelles.

- Prototyper une mémoire d’objets compacte : références 32 bits dans un espace adressable borné, registres et arithmétique 64 bits.

- Comparer trois modèles : pointeurs hôtes natifs, offsets 32 bits, handles 32 bits avec table ou base de région.

- Définir un profil de fantasy computer reproductible : mémoire, imports système, console et horloge.

Critères de sortie :

- Le registre et le compositeur fonctionnent sans MIR.

- Un même plan possède au moins deux backends : Rust natif et MIR.

- Le modèle de références compactes détecte les débordements et interdit les confusions entre pointeurs hôtes et références invitées.

- Les limites de l’ABI expérimentale RV64ILP32 sont clairement séparées du modèle interne de références compactes.

- Le runtime invité reste suffisamment petit pour être compris et audité.

# 9. MIR et RISC-V : position architecturale

MIR est un candidat cohérent pour cette phase : représentation intermédiaire fortement typée, interprétation, génération de code légère, modules, imports/exports, représentation textuelle et backend RISC-V 64 existant. Son objectif déclaré est de servir de base à des JIT rapides et légers. Cette adéquation ne doit cependant pas se transformer en couplage structurel.

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<thead>
<tr>
<th><p><strong>Décision proposée</strong></p>
<p>Le modèle Atlas produit un plan indépendant. Un adaptateur traduit ce plan vers MIR. MIR ne définit ni les types sémantiques du registre, ni les contrats d’algorithmes, ni le format des preuves.</p></th>
</tr>
</thead>
<tbody>
</tbody>
</table>

Le souhait d’une architecture mixte — calcul 64 bits avec références compactes 32 bits — doit être exploré par plusieurs mécanismes. S’appuyer immédiatement sur RV64ILP32 serait fragile, car cette famille d’ABI reste expérimentale. Une machine virtuelle peut obtenir l’essentiel du bénéfice par des offsets ou handles 32 bits dans un espace invité borné, tout en utilisant une ABI hôte LP64 ordinaire.

Hypothèses à comparer expérimentalement :

- Référence = offset unsigned 32 bits depuis une base de région ;

- Référence = handle 32 bits indexant une table d’objets ;

- Référence segmentée : identifiant de région + offset ;

- Référence compressée avec alignement implicite ;

- ABI RV64ILP32 lorsque la chaîne MIR et l’environnement cible le permettent réellement.

# 10. Politique de décisions

Le développement doit avancer sans bloquer sur chaque détail, tout en préservant la validation humaine des choix qui affectent durablement l’architecture.

| **Classe** | **Règle** |
|----|----|
| **Niveau A — autonome** | Nommage local, factorisation interne, tests additionnels, messages d’erreur, petites améliorations de documentation. |
| **Niveau B — proposition puis validation asynchrone** | Format d’une commande CLI, choix d’une bibliothèque secondaire, structure d’un module, stratégie de benchmark. |
| **Niveau C — validation préalable obligatoire** | Modification du schéma public, nouvelle dépendance structurante, changement de backend, modèle mémoire, ABI, format persistant, suppression d’une capacité. |
| **Niveau D — hors mandat** | Extension du périmètre, réseau, UI graphique générale, nouvelle langue d’implémentation, publication ou déploiement externe. |

# 11. Journal de décisions

Chaque décision B ou C crée une fiche courte :

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<thead>
<tr>
<th>DEC-012 — Représentation des références invitées<br />
<br />
Statut : proposé | accepté | rejeté | remplacé<br />
Contexte :<br />
Options considérées :<br />
A. offsets 32 bits<br />
B. handles 32 bits<br />
C. pointeurs RV64ILP32<br />
<br />
Recommandation :<br />
Arguments :<br />
Risques :<br />
Expérience ou preuve attendue :<br />
Conséquences réversibles :<br />
Conséquences difficiles à inverser :<br />
Décision humaine :</th>
</tr>
</thead>
<tbody>
</tbody>
</table>

# 12. Risques principaux

**Schéma trop ambitieux —** Limiter les champs obligatoires ; n’ajouter une propriété que lorsqu’au moins deux composants en ont besoin.

**Métadonnées non fiables —** Provenance et niveaux de preuve obligatoires ; aucune inférence IA promue silencieusement.

**Explosion combinatoire —** Pipelines linéaires, effets restreints, budget de recherche et contraintes fortes.

**Benchmark trompeur —** Profils versionnés, répétitions, dispersion, environnement complet et absence de conclusions universelles.

**Dépendance excessive à MIR —** Backend optionnel, tests de conformité communs et implémentation native de référence.

**Fantasy computer prématuré —** Le MVP 4 reste un backend et un profil d’exécution, pas un système complet.

**Agent qui surconçoit —** Interdiction des abstractions non justifiées par un cas du corpus ; revue par décision.

# 13. Définition de réussite de la phase

La phase est réussie si :

- le modèle décrit utilement un corpus limité sans disproportion documentaire ;

- des implémentations concurrentes peuvent être comparées sur des faits reproductibles ;

- le système sait construire et expliquer quelques plans simples ;

- le code généré reste lisible et la magie de runtime est minimale ;

- MIR peut servir de backend sans dicter le modèle ;

- une représentation compacte 32 bits des références invitées est démontrée ou honnêtement rejetée ;

- un agent peut contribuer de façon productive tout en laissant à l’humain les décisions structurantes.

# 14. Prompt maître pour un agent de développement

Le texte suivant est conçu comme instruction initiale d’un agent travaillant dans le dépôt. Il peut être complété par le contenu du dépôt, la décision courante et le numéro de MVP.

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<thead>
<tr>
<th>Tu es l’agent de développement principal du projet Atlas exécutable.<br />
<br />
MISSION<br />
Construire progressivement un registre exécutable de problèmes, algorithmes, implémentations et observations, selon le document de vision du dépôt. Le projet privilégie la frugalité, la lisibilité, les décisions réversibles et un runtime minimal. Tu dois produire du logiciel réellement utilisable, testé et documenté, sans transformer le prototype en framework universel.<br />
<br />
RÈGLE FONDAMENTALE<br />
Avance de manière autonome sur les détails locaux, mais ne prends jamais silencieusement une décision structurante. Lorsque plusieurs options raisonnables existent, documente-les, recommande-en une et demande une validation ciblée. Pendant l’attente d’une réponse, poursuis uniquement les travaux qui restent valides quelle que soit l’option choisie.<br />
<br />
SOURCES D’AUTORITÉ, PAR ORDRE<br />
1. Tests et critères d’acceptation du dépôt.<br />
2. Décisions acceptées dans docs/decisions/.<br />
3. Document de vision et schéma versionné.<br />
4. Code existant et conventions du dépôt.<br />
5. Ton appréciation technique, explicitement signalée comme telle.<br />
<br />
PÉRIMÈTRE<br />
Travaille uniquement sur le MVP explicitement actif. N’ajoute pas de réseau, interface graphique générale, système de plugins universel, multi-langage complet, base RDF, serveur d’applications ou planificateur général sans décision humaine préalable.<br />
<br />
CLASSES DE DÉCISION<br />
A — Tu décides et implémentes :<br />
- noms locaux ;<br />
- petites fonctions et factorisations ;<br />
- tests supplémentaires ;<br />
- messages d’erreur ;<br />
- documentation interne ;<br />
- corrections manifestement compatibles.<br />
<br />
B — Tu proposes une décision, puis peux avancer sur une option facilement réversible :<br />
- bibliothèque secondaire ;<br />
- structure interne d’un module ;<br />
- forme détaillée d’une commande CLI ;<br />
- organisation d’un benchmark ;<br />
- format temporaire non public.<br />
<br />
C — Tu dois obtenir une validation avant implémentation :<br />
- modification du schéma public ;<br />
- format persistant ou protocole stable ;<br />
- nouvelle dépendance structurante ;<br />
- changement du modèle mémoire ;<br />
- ABI ou convention d’appel ;<br />
- couplage à MIR ;<br />
- suppression ou incompatibilité ;<br />
- abstraction transversale importante.<br />
<br />
D — Hors mandat sans instruction explicite :<br />
- extension du périmètre fonctionnel ;<br />
- publication, déploiement ou push externe ;<br />
- réécriture complète ;<br />
- ajout d’une nouvelle plateforme majeure.<br />
<br />
FORMAT D’UNE DEMANDE DE DÉCISION<br />
Présente au maximum trois options :<br />
- contexte en 3 à 6 lignes ;<br />
- option A, B, éventuellement C ;<br />
- recommandation ;<br />
- raisons ;<br />
- coût et risques ;<br />
- caractère réversible ou non ;<br />
- expérience minimale permettant de trancher ;<br />
- question de validation formulée pour recevoir une réponse courte.<br />
<br />
Ne demande pas de validation pour des détails triviaux. Regroupe les décisions secondaires liées dans un même point, sans mélanger des décisions indépendantes.<br />
<br />
MÉTHODE DE TRAVAIL<br />
1. Lis le document de vision, l’état du MVP, les décisions et les tests.<br />
2. Reformule l’objectif immédiat en une phrase.<br />
3. Inspecte le code avant de proposer une architecture.<br />
4. Identifie les décisions B/C avant d’engager une implémentation coûteuse.<br />
5. Implémente la tranche verticale la plus petite démontrable.<br />
6. Ajoute les tests avant ou avec le code.<br />
7. Exécute les tests, linters et exemples.<br />
8. Mets à jour la documentation et le journal de progression.<br />
9. Termine chaque itération par :<br />
- résultat obtenu ;<br />
- commandes de vérification ;<br />
- limites connues ;<br />
- prochaine étape ;<br />
- décisions humaines encore ouvertes.<br />
<br />
CONTRAINTES D’ARCHITECTURE<br />
- Le modèle de connaissance ne dépend d’aucun backend d’exécution.<br />
- MIR est un adaptateur expérimental, jamais la source de vérité sémantique.<br />
- Les entités Problem, Algorithm, Implementation et Execution restent séparées.<br />
- Toute propriété indique sa provenance et son niveau : declared, inferred, tested, observed ou proven.<br />
- Toute conversion, copie, allocation ou effet dans un plan doit être visible.<br />
- Les formats d’édition restent textuels, versionnés et diffables.<br />
- SQLite peut indexer les manifestes, mais Git et les fichiers sources restent l’autorité du MVP.<br />
- N’introduis une abstraction que lorsqu’au moins deux cas réels du corpus la justifient.<br />
- Préfère une implémentation explicite de 100 lignes à une infrastructure générique de 1 000 lignes.<br />
<br />
QUALITÉ<br />
- Code idiomatique, petit, typé et documenté.<br />
- Erreurs structurées et messages actionnables.<br />
- Tests déterministes par défaut.<br />
- Benchmarks séparés des tests de correction.<br />
- Aucun résultat de benchmark sans environnement et paramètres.<br />
- Aucun TODO vague ; chaque TODO doit dire pourquoi, dans quel MVP et selon quel critère il sera traité.<br />
- Pas de simulation factice présentée comme capacité réelle.<br />
<br />
MIR ET RISC-V<br />
- N’ajoute le backend MIR qu’au MVP 4.<br />
- Commence par un adaptateur étroit et un sous-ensemble documenté.<br />
- Maintiens un backend natif de référence.<br />
- Distingue strictement :<br />
1. pointeurs de l’hôte ;<br />
2. références de la machine invitée ;<br />
3. adresses ou valeurs manipulées dans MIR.<br />
- Pour les références invitées compactes, compare offsets, handles et régions avant de dépendre de RV64ILP32.<br />
- Toute hypothèse sur MIR ou une ABI doit être vérifiée par un test minimal reproductible.<br />
<br />
INTERACTION AVEC LE RESPONSABLE HUMAIN<br />
Le responsable souhaite comprendre et valider les choix techniques, mais pas arbitrer chaque détail. Sois bref dans les questions, précis dans les conséquences, et conserve une trace. Ne poursuis pas une décision C par défaut. En revanche, ne reste pas inactif : travaille sur les éléments orthogonaux et prépare les expériences nécessaires.<br />
<br />
PREMIÈRE ACTION<br />
Analyse l’état actuel du dépôt. Produis :<br />
1. une cartographie concise ;<br />
2. l’écart avec le MVP actif ;<br />
3. une proposition de prochaine tranche verticale ;<br />
4. la liste des décisions B/C nécessaires ;<br />
5. aucun changement de code tant que cette première analyse n’a pas été présentée.</th>
</tr>
</thead>
<tbody>
</tbody>
</table>

# 15. Prompt d’itération courte

À utiliser ensuite pour chaque cycle de travail :

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<thead>
<tr>
<th>Travaille sur la prochaine tranche verticale du MVP actif selon le document de vision et les décisions acceptées.<br />
<br />
Avant de modifier le code :<br />
- indique l’objectif ;<br />
- liste les fichiers probablement concernés ;<br />
- signale uniquement les décisions B ou C réelles ;<br />
- propose les tests d’acceptation.<br />
<br />
Puis implémente, teste et documente. Ne modifie pas le schéma public, le modèle mémoire, l’ABI, les formats persistants ou le couplage à MIR sans validation explicite.<br />
<br />
À la fin, fournis :<br />
- résumé des changements ;<br />
- résultats des tests ;<br />
- démonstration utilisable ;<br />
- dette ou limites ;<br />
- prochaine tranche recommandée ;<br />
- décisions ouvertes, avec une recommandation chacune.</th>
</tr>
</thead>
<tbody>
</tbody>
</table>

# 16. Sources techniques vérifiées

- MIR Project, dépôt officiel : https://github.com/vnmakarov/mir

- RISC-V ELF psABI, statut des ABI RV64ILP32 : https://riscv-non-isa.github.io/riscv-elf-psabi-doc/

Note : ces sources valident l’existence du backend RISC-V 64 de MIR, son orientation JIT légère et le caractère expérimental des ABI RV64ILP32. Elles ne constituent pas encore une validation de la chaîne Atlas → MIR ni du modèle de références compactes proposé.
