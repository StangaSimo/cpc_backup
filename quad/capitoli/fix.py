import re

# Nome del tuo file (cambia se necessario)
INPUT_FILE = "godo2.tex"
OUTPUT_FILE = "godo2_fixed.tex"

def replace_images(content):
    # Cerca il pattern \includegraphics[scale=...]{...}
    # Cattura il valore dello scale (gruppo 1) e il nome del file (gruppo 2)
    pattern = r"\\includegraphics\[scale=([\d\.]+)\]\{(.*?)\}"

    def replacer(match):
        scale = float(match.group(1))
        filename = match.group(2)
        
        # Calcola l'altezza in cm basandosi sullo scale (euristica: scale 0.5 ~ 7cm)
        # Moltiplico per 14 per avere un box ben visibile che riempia lo spazio
        height_cm = scale * 14
        if height_cm < 1.0: height_cm = 1.0 # Minimo 1cm

        # Escape degli underscore per il testo LaTeX (altrimenti da errore)
        display_name = filename.replace("_", "\\_")

        # Crea il blocco sostitutivo
        # Usa minipage per riservare lo spazio verticale corretto
        replacement = (
            f"\\fbox{{\\begin{{minipage}}[c][{height_cm:.1f}cm]{{\\linewidth}}"
            f"\\centering \\tiny \\textbf{{IMG MANCANTE}} \\\\ {display_name}"
            f"\\end{{minipage}}}}"
        )
        return replacement

    return re.sub(pattern, replacer, content)

try:
    with open(INPUT_FILE, 'r', encoding='utf-8') as f:
        content = f.read()
    
    new_content = replace_images(content)
    
    with open(OUTPUT_FILE, 'w', encoding='utf-8') as f:
        f.write(new_content)
        
    print(f"Fatto! File salvato come: {OUTPUT_FILE}")

except FileNotFoundError:
    print(f"Errore: Non trovo il file {INPUT_FILE}. Assicurati di averlo salvato nella cartella.")
