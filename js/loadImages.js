import Kick from "./images/Kick.png";
import NinjaSword from "./images/NinjaSword.png";
import Nunchucks from "./images/Nunchucks.png";
import ShadowFireball from "./images/ShadowFireball.png";
import ShadowSlip from "./images/ShadowSlip.png";
import RunInCircles from "./images/RunInCircles.png";
import LightningFastKarateChop from "./images/LightningFastKarateChop.png";
import Rampage from "./images/Rampage.png";
import Muscle from "./images/Muscle.png";
import Zap from "./images/Zap.png";
import Regenerate from "./images/Regenerate.png";
import Gravedigger from "./images/Gravedigger.png";
import ZombieCorps from "./images/ZombieCorps.png";
import Apocalypse from "./images/Apocalypse.png";
import SamuraiSword from "./images/SamuraiSword.png";
import Helmet from "./images/Helmet.png";
import Smash from "./images/Smash.png";
import Lightning from "./images/Lightning.png";
import Earthquake from "./images/Earthquake.png";
import Twist from "./images/Twist.png";
import Bend from "./images/Bend.png";
import JugglingKnives from "./images/JugglingKnives.png";
import AcidSpray from "./images/AcidSpray.png";
import Nose from "./images/Nose.png";
import BackwardsMoustachio from "./images/BackwardsMoustachio.png";
import NoseOfTheTaunted from "./images/NoseOfTheTaunted.png";
import MustacheMash from "./images/MustacheMash.png";
import BigHairyDeal from "./images/BigHairyDeal.png";

import NoBooster from "./images/NoBooster.png";

import "./shims";

const srcs = {
  Kick,
  NinjaSword,
  Nunchucks,
  ShadowFireball,
  ShadowSlip,
  RunInCircles,
  LightningFastKarateChop,
  Rampage,
  Muscle,
  Zap,
  Regenerate,
  Gravedigger,
  ZombieCorps,
  Apocalypse,
  SamuraiSword,
  Helmet,
  Smash,
  Lightning,
  Earthquake,
  Twist,
  Bend,
  JugglingKnives,
  AcidSpray,
  Nose,
  BackwardsMoustachio,
  NoseOfTheTaunted,
  MustacheMash,
  BigHairyDeal,
  NoBooster
};

function loadImages() {
  const entries = Object.entries(srcs).map(
    ([key, src]) =>
      new Promise((resolve, reject) => {
        const img = new Image();
        img.src = src;
        img.addEventListener("load", () => resolve([key, img]));
        img.addEventListener("error", reject);
      })
  );
  return Promise.all(entries).then(Object.fromEntries);
}

export default loadImages;
