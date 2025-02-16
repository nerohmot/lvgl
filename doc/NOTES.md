# notes

- cipal heeft geen KBO nummer in het bestand ... -> kijken of de mensen (ISSZ=rijksregister nummers) overeenkomen
- dmfa & bosa formaat hebben alle twee KBO nummers (=BTW nummer van de instelling)
- cipal = 1 lijn header
- bosa = 1 lijn header
- dmfa = 2 lijnen header, eerste = nederlands, tweede = frans
- bedragen zijn in het nederlands (duizend separator = punt '.' & decimal separator = comma ',')
- KBO nummer heeft altijd 9 cijfers -> u32
- INSZ = rijksregister nummer = 11 posities
  - YYMMDD



  ## DMFA
  kwart, WGC, WNK, INSZ, LC, LC_bedr
  WGC = Werkgever cathegorie -> statutair of contractueel of ... de hoogte van de bijdrage hangt daaraan vast
  WNK = Werknemers kengetal -> statuut van de werknemer
  INSZ = rijksregister nummer
  LC = loon code -> wedde, vakantiegeld, reiskosten, ...
  LC_bedr = brutto loon

  