# Upravljanje Lozinkama

Ovaj projekat je implementacija jednostavnog sistema za upravljanje lozinkama korišćenjem programskog jezika Rust. 
Projekat je razvijen kao deo predmeta **Upravljanje projektima u industriji i nauci**.

## Opis projekta

Aplikacija omogućava korisnicima da:  
- Dodaju lozinke za različite servise.  
- Šifruju i bezbedno čuvaju lozinke koristeći AES-256-GCM enkripciju.  
- Preuzmu šifrovane lozinke za određene servise nakon što su ih dodali.  
- Čuvaju lozinke u JSON fajlu na disku.  

## Kako koristiti

1. Instalirajte Rust ako već nije instaliran. Preuzmite ga sa [zvanične stranice](https://www.rust-lang.org/).  
2. Klonirajte ovaj repozitorijum na svoj računar.  
3. Pokrenite aplikaciju koristeći sledeću komandu:  

    ```bash
    cargo run
    ```

4. Pratite uputstva u terminalu za dodavanje i preuzimanje lozinki.

## Napomena

- Nemojte ručno menjati sadržaj fajlova `passwords.json` ili `key.bin`, jer to može dovesti do gubitka podataka.  
- U slučaju gubitka fajla `key.bin`, šifrovane lozinke više neće biti moguće dešifrovati.  
