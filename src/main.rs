#![allow(dead_code)]

/*
*Klassen ApendoNumberEchoChamber, rust har inget "class" nyckelord eftersom det är valbart om man vill programmera objekt-orienterat eller inte
*
*Har 3 variabler, usize och isize får sin storlek (8-32-64 bit, etc) från systemet programmet kompileras på, dessa kommer troligen bli i64 och u64.
*De får sin mutabilitet från den som skapar objektet, väljer man "mut" nyckelordet när man skapar en instans av klassen kommer det gå att ändra på variablerna utifrån, om inte måste man hålla sig till set/get funktioner.
*/
struct ApendoNumberEchoChamber
{
   step_length: usize,
   apendo_mod: isize,
   apendo_count: usize
}


/*
*Dessa är funtionerna för klassen.
*/
impl ApendoNumberEchoChamber
{
   /*
   *Seriell utskrivning av inmatningen som är en tuple av isize, första objektet nårs med .0 , etc.
   *Denna funktionen kollar om inmatningen kommer med det större talet först eller ej eftersom rusts Range objekt inte gillar att man itererar från ett större tal till ett mindre tal.
   ***Är det första inmatade talet större än det andra byter funktionen plats på parametrarna och skickar vidare dem till utskrifts-funktionen.
   */
   fn serial_echo(&mut self, mut bound: (isize,isize))
   {
      let mut reverse = false;
      if bound.0>bound.1
      {
         std::mem::swap(&mut bound.0, &mut bound.1);
         reverse = true;
      }
      let range = bound.0..(bound.1+1);
      if reverse
         {self.print_range(range.rev());}
      else
         {self.print_range(range);}
   }

   /*
   *Utskrivning av en specific position av inmatningen som är en tuple av isize, första objektet nårs med .0 , etc.
   *Denna funktionen kollar om inmatningen kommer med det större talet först eller ej eftersom rusts Range objekt inte gillar att man itererar från ett större tal till ett mindre tal.
   ***Är det första inmatade talet större än det andra byter funktionen plats på parametrarna och skickar vidare dem till utskrifts-funktionen.
   */
   fn specific_echo(&mut self, mut bound: (isize,isize), number_to_print: isize)
   {
      let mut reverse = false;
      if bound.0>bound.1
      {
         std::mem::swap(&mut bound.0, &mut bound.1);
         reverse = true;
      }
      let range = bound.0..(bound.1+1);
      if reverse
         {self.print_nth_element(range.rev(), number_to_print);}
      else
         {self.print_nth_element(range, number_to_print);}
   }
   
   /*
   *Seriell utskrivning, tar en generisk range-iterator eftersom .rev() returnerar en Rev<Iterator<Range>> istället för en Iterator<Range>
   *Återställer räkningen av Apendo-utskrifter, om apendo_mod är 0 så ignoreras värdet.
   *Itererar igenom hela iteratorn och skriver ut enligt uppgift, Apendo om värdet i rangen är jämnt delbart med 0, värdet annars.
   */
   fn print_range<T: Iterator<Item=isize>> (&mut self, iter: T)
   {
      self.apendo_count = 0;
      for (iteration, value) in iter.enumerate()
      {
         if iteration%self.step_length == 0
         {
            if self.apendo_mod != 0 && value%self.apendo_mod == 0
               {
                  println!("Apendo");
                  self.apendo_count+=1;
               }
            else
               {println!("{}", value);}
         }
      }
   }
   
   /*
   *Specifik utskrift, tar en generisk range-iterator eftersom .rev() returnerar en Rev<Iterator<Range>> istället för en Iterator<Range>
   *Återställer räkningen av Apendo-utskrifter, om apendo_mod är 0 så ignoreras värdet.
   *Itererar igenom hela iteratorn och skriver ut enligt uppgift, Apendo om värdet i rangen är jämnt delbart med 0, värdet annars.
   *
   *Skillnaden mellan föregående är att bara det N:te värdet (eller Apendo) skrivs ut.
   */
   fn print_nth_element<T: Iterator<Item=isize>> (&mut self, iter: T, number_to_print: isize)
   {
      self.apendo_count = 0;
      for (iteration, value) in iter.enumerate()
      {
         if iteration%self.step_length == 0
         {
            if self.apendo_mod != 0 && value%self.apendo_mod == 0
               {
                  println!("Apendo");
                  self.apendo_count+=1;
               }
            else if iteration as isize == number_to_print-1
               {println!("{}", value);}
         }
      }
   }
   
   fn print_apendo_count(&self)
   {
      println!("Last time a function in this instance was used \"Apendo\" was printed {} times.", self.apendo_count)
   }
   
   fn set_step_length(&mut self, length:usize)
   {
      if length != 0
         {self.step_length = length;}
   }
   
   fn set_apendo_mod(&mut self, apendo:isize)
   {
      if apendo > 0
         {self.apendo_mod = apendo;}
   }
   
   /*
   *Get funktionerna returnerar kopior av värdena och inte värdena.
   */
   fn get_step_length(&self)
      -> usize
   {
      self.step_length
   }
   
   fn get_apendo_mod(&self)
      -> isize
   {
      self.apendo_mod
   }

   /*
   *new() är det närmaste man kommer till en konstruktor i rust, det är vida använt men måste inte heta new(), det kan ta parametrar men till skillnad från de andra funktionerna så tar den inte en referens till self, vilket gör att det inte kallas på en existerande instans utan på klass-namnet själv.
   *ApendoNumberEchoChamber::new()
   */
   fn new()
      -> ApendoNumberEchoChamber
   {
      ApendoNumberEchoChamber{step_length: 1, apendo_mod: 0, apendo_count: 0}
   }
   
   /*
   *Samma som new() men med parametrar medskickade, vill man ha default-värden på parametrarna måste man implementera attributen Default.
   */
   fn new_with_parameters(length: usize, a_mod: isize, a_count: usize)
      -> ApendoNumberEchoChamber
   {
      ApendoNumberEchoChamber{step_length: length, apendo_mod: a_mod, apendo_count: a_count}
   }
}

fn main()
{
   let mut chamber = ApendoNumberEchoChamber::new();
   //let mut chamber = ApendoNumberEchoChamber::new_with_parameters(1, 0, 0);
   chamber.serial_echo((5,15));
   //chamber.set_apendo_mod(2);
   //chamber.set_step_length(3);
   //println!("{}", chamber.get_apendo_mod());
   //println!("{}", chamber.get_step_length());
   //chamber.serial_echo((5,3));
   //chamber.serial_echo((3,5));
   //chamber.serial_echo((-5, 3));
   //chamber.serial_echo((-5, -3));
   //chamber.serial_echo((-3, -5));
   //chamber.specific_echo((16, 12),3);
   chamber.print_apendo_count();
}
